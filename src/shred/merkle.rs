use std::ops::Range;

use solana_sdk::{
    hash::{hashv, Hash},
    packet::PACKET_DATA_SIZE,
};
use thiserror::Error;

use super::{
    layout::{self, SIZE_OF_SIGNATURE},
    verify_shred::SIZE_OF_NONCE,
    DataShredHeader, ShredCommonHeader,
};

const SIZE_OF_MERKLE_PROOF_ENTRY: usize = 20;
const MERKLE_HASH_PREFIX_NODE: &[u8] = b"\x01SOLANA_MERKLE_SHREDS_NODE";
const MERKLE_HASH_PREFIX_LEAF: &[u8] = b"\x00SOLANA_MERKLE_SHREDS_LEAF";
const SIZE_OF_HEADERS: usize = 88;
const SIZE_OF_PAYLOAD: usize =
    PACKET_DATA_SIZE - SIZE_OF_NONCE - SIZE_OF_HEADERS + SIZE_OF_SIGNATURE;
const SIZE_OF_MERKLE_ROOT: usize = 32;

pub struct ShredData {
    pub common_header: ShredCommonHeader,
    pub data_header: DataShredHeader,
    pub payload: Vec<u8>,
}
pub struct ShredCode {
    pub common_header: ShredCommonHeader,
    pub data_header: DataShredHeader,
    pub payload: Vec<u8>,
}

pub enum Shred {
    ShredCode(ShredCode),
    ShredData(ShredData),
}

//get merkle root

#[derive(Debug, Error)]
enum MerkleRootErrors {
    #[error("Invalid merkle proof")]
    InvalidMerkleProof,
    #[error("Invalid proof size")]
    InvalidProofSize,
    #[error("Invalid payload size")]
    InvalidPayloadSize,
}

type MerkleProofEntry = [u8; 20];

fn get_merkle_root<'a, I>(index: usize, node: Hash, proof: I) -> Result<Hash, MerkleRootErrors>
where
    I: IntoIterator<Item = &'a MerkleProofEntry>,
{
    let (index, root) = proof
        .into_iter()
        .fold((index, node), |(index, node), other| {
            let parent = if index % 2 == 0 {
                join_nodes(node, other)
            } else {
                join_nodes(other, node)
            };
            (index >> 1, parent)
        });
    (index == 0)
        .then_some(root)
        .ok_or(MerkleRootErrors::InvalidMerkleProof)
}

fn capacity(proof_size: u8, chained: bool, resigned: bool) -> Result<usize, MerkleRootErrors> {
    SIZE_OF_PAYLOAD
        .checked_sub(
            SIZE_OF_HEADERS
                + if chained { SIZE_OF_MERKLE_ROOT } else { 0 }
                + usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY
                + if resigned { SIZE_OF_SIGNATURE } else { 0 },
        )
        .ok_or(MerkleRootErrors::InvalidProofSize)
}

fn get_proof_offset(
    proof_size: u8,
    chained: bool,
    resigned: bool,
) -> Result<usize, MerkleRootErrors> {
    Ok(SIZE_OF_HEADERS
        + capacity(proof_size, chained, resigned)?
        + if chained { SIZE_OF_MERKLE_ROOT } else { 0 })
}

fn join_nodes<S: AsRef<[u8]>, T: AsRef<[u8]>>(node: S, other: T) -> Hash {
    let node = &node.as_ref()[..SIZE_OF_MERKLE_PROOF_ENTRY];
    let other = &other.as_ref()[..SIZE_OF_MERKLE_PROOF_ENTRY];
    hashv(&[MERKLE_HASH_PREFIX_NODE, node, other])
}

fn get_merkle_proof(
    shred: &[u8],
    proof_offset: usize,
    proof_size: u8,
) -> Result<impl Iterator<Item = &MerkleProofEntry>, MerkleRootErrors> {
    let proof_size = usize::from(proof_size) * SIZE_OF_MERKLE_PROOF_ENTRY;
    Ok(shred
        .get(proof_offset..proof_offset + proof_size)
        .ok_or(MerkleRootErrors::InvalidPayloadSize)?
        .chunks(SIZE_OF_MERKLE_PROOF_ENTRY)
        .map(<&MerkleProofEntry>::try_from)
        .map(Result::unwrap))
}

fn get_merkle_node(shred: &[u8], offsets: Range<usize>) -> Result<Hash, MerkleRootErrors> {
    let node = shred
        .get(offsets)
        .ok_or(MerkleRootErrors::InvalidPayloadSize)?;
    Ok(hashv(&[MERKLE_HASH_PREFIX_LEAF, node]))
}

pub fn get_merkle_root_from_shred(
    shred: &[u8],
    proof_size: u8,
    chained: bool,
    resigned: bool,
) -> Option<Hash> {
    let index = {
        let fec_set_index = <[u8; 4]>::try_from(shred.get(79..83)?)
            .map(u32::from_le_bytes)
            .ok()?;
        layout::get_index(shred)?
            .checked_sub(fec_set_index)
            .map(usize::try_from)?
            .ok()?
    };

    let proof_offset = get_proof_offset(proof_size, chained, resigned).ok()?;
    let proof = get_merkle_proof(shred, proof_offset, proof_size).ok()?;
    let node = get_merkle_node(shred, SIZE_OF_SIGNATURE..proof_offset).ok()?;
    get_merkle_root(index, node, proof).ok()
}

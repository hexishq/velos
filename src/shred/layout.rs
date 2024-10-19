use std::ops::Range;

use solana_sdk::{clock::Slot, hash::Hash, signature::Signature};

use super::ShredVariant;

const SIZE_OF_SIGNATURE: usize = 64;
const SIZE_OF_SHRED_VARIANT: usize = 1;
const OFFSET_OF_SHRED_SLOT: usize = SIZE_OF_SIGNATURE + SIZE_OF_SHRED_VARIANT;
const OFFSET_OF_SHRED_VARIANT: usize = SIZE_OF_SIGNATURE;
const SIZE_OF_PAYLOAD: usize = 1228;
const SIGNED_MESSAGE_OFFSETS: Range<usize> = SIZE_OF_SIGNATURE..SIZE_OF_PAYLOAD;
enum SignedData<'a> {
    Chunk(&'a [u8]), // Chunk of payload past signature.
    MerkleRoot(Hash),
}

pub fn get_slot(shred: &[u8]) -> Option<Slot> {
    <[u8; 8]>::try_from(shred.get(OFFSET_OF_SHRED_SLOT..)?.get(..8)?)
        .map(Slot::from_le_bytes)
        .ok()
}

pub fn get_signature(shred: &[u8]) -> Option<Signature> {
    shred
        .get(..SIZE_OF_SIGNATURE)
        .map(Signature::try_from)?
        .ok()
}

pub fn get_shred_variant(shred: &[u8]) -> Option<ShredVariant> {
    let Some(&shred_variant) = shred.get(OFFSET_OF_SHRED_VARIANT) else {
        return None;
    };
    match ShredVariant::from_u8(shred_variant) {
        Ok(variant) => return Some(variant),
        Err(_) => return None,
    };
}

pub fn get_signed_data(shred: &[u8]) -> Option<SignedData> {
    let shred_variant = get_shred_variant(shred)?;

    match shred_variant {
        ShredVariant::LegacyCode | ShredVariant::LegacyData => {
            let chunk = shred.get(SIGNED_MESSAGE_OFFSETS)?;
            return Some(SignedData::Chunk(chunk));
        }

        ShredVariant::MerkleCode {
            proof_size,
            chained,
            resigned,
        } => {}
        ShredVariant::MerkleData {
            proof_size,
            chained,
            resigned,
        } => todo!(),
    }

    None
}

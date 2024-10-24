use hexis_shred::shred::ShredType;
use serde::{Deserialize, Serialize};
use solana_sdk::{clock::Slot, pubkey::Pubkey};

pub type DuplicateShredIndex = u16;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DuplicateShred {
    pub from: Pubkey,
    pub wallclock: u64,
    pub slot: Slot,
    _unused: u32,
    _unused_shred_type: ShredType,
    // Serialized DuplicateSlotProof split into chunks.
    num_chunks: u8,
    chunk_index: u8,
    chunk: Vec<u8>,
}

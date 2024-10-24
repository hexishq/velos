use bv::BitVec;
use serde::{Deserialize, Serialize};
use solana_sdk::{clock::Slot, hash::Hash, pubkey::Pubkey};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct RestartLastVotedForkSlots {
    pub from: Pubkey,
    pub wallclock: u64,
    offsets: SlotsOffsets,
    pub last_voted_slot: Slot,
    pub last_voted_hash: Hash,
    pub shred_version: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
enum SlotsOffsets {
    RunLengthEncoding(RunLengthEncoding),
    RawOffsets(RawOffsets),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
struct RunLengthEncoding(Vec<u16>);

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
struct RawOffsets(BitVec<u8>);

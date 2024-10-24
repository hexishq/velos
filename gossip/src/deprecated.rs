use serde::{Deserialize, Serialize};
use solana_sdk::clock::Slot;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EpochIncompleteSlots {
    first: Slot,
    compression: CompressionType,
    compressed_list: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
enum CompressionType {
    Uncompressed,
    GZip,
    BZip2,
}

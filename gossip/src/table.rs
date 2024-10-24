//! Cluster Replicated Data Store
//! Stores gossip data
//! Work in progress...
//? References:
// 1. https://github.com/Syndica/sig/blob/main/src/gossip/table.zig
// 2. https://github.com/anza-xyz/agave/blob/master/gossip/src/crds.rs

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    sync::{Mutex, RwLock},
};

use indexmap::{IndexMap, IndexSet};
use lru::LruCache;
use solana_sdk::{hash::Hash, pubkey::Pubkey};

pub struct GossipTable {
    /// Stores the map of labels and values
    // table: IndexMap<CrdsValueLabel, VersionedCrdsValue>,
    cursor: Cursor, // Next insert ordinal location.
    // shards: CrdsShards,
    nodes: IndexSet<usize>, // Indices of nodes' ContactInfo.
    // Indices of Votes keyed by insert order.
    votes: BTreeMap<u64 /*insert order*/, usize /*index*/>,
    // Indices of EpochSlots keyed by insert order.
    epoch_slots: BTreeMap<u64 /*insert order*/, usize /*index*/>,
    // Indices of DuplicateShred keyed by insert order.
    duplicate_shreds: BTreeMap<u64 /*insert order*/, usize /*index*/>,
    // Indices of all crds values associated with a node.
    records: HashMap<Pubkey, IndexSet<usize>>,
    // Indices of all entries keyed by insert order.
    entries: BTreeMap<u64 /*insert order*/, usize /*index*/>,
    // Hash of recently purged values.
    purged: VecDeque<(Hash, u64 /*timestamp*/)>,
    // Mapping from nodes' pubkeys to their respective shred-version.
    shred_versions: HashMap<Pubkey, u16>,
}

impl GossipTable {
    pub fn new() {}
}

#[derive(PartialEq, Eq, Debug)]
pub enum GossipTableError {
    DuplicatePush(/*num dups:*/ u8),
    InsertFailed,
    UnknownStakes,
}

#[derive(Clone, Copy)]
pub enum GossipRoute<'a> {
    LocalMessage,
    PullRequest,
    PullResponse,
    PushMessage(/*from:*/ &'a Pubkey),
}

/// A cursor is a mechanism used to track the ordinal position or the insert order of elements within the GossipTable
/// Specifically, it helps keep track of the next insertion point and updates accordingly as new elements are added.
#[derive(Clone, Copy, Default)]
pub struct Cursor(u64);

impl Cursor {
    // returns the current value of the cursor
    fn ordinal(&self) -> u64 {
        self.0
    }

    // Updates the cursor position given the ordinal index of value consumed.
    // method updates the cursor by advancing it when a new entry is consumed or inserted, ensuring that it tracks the maximum ordinal index it has seen.
    #[inline]
    fn consume(&mut self, ordinal: u64) {
        self.0 = self.0.max(ordinal + 1);
    }
}

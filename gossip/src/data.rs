use {
    crate::{
        contact_info::ContactInfo,
        duplicate_shred::{DuplicateShred, DuplicateShredIndex},
        legacy_contact_info::LegacyContactInfo,
    },
    bincode::serialize,
    bv::BitVec,
    serde::{Deserialize, Serialize},
    solana_sdk::{
        clock::Slot,
        hash::Hash,
        pubkey::Pubkey,
        signature::{Keypair, Signable, Signature},
        transaction::Transaction,
    },
    std::{
        borrow::{Borrow, Cow},
        collections::BTreeSet,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GossipTableValue {
    pub signature: Signature,
    pub data: GossipTableData,
}

impl GossipTableValue {
    fn new_unsigned(data: GossipTableData) -> Self {
        Self {
            signature: Signature::default(),
            data,
        }
    }

    pub fn new_signed(data: GossipTableData, keypair: &Keypair) -> Self {
        let mut value = Self::new_unsigned(data);
        value.sign(keypair);
        value
    }

    pub fn pubkey(&self) -> Pubkey {
        match &self.data {
            GossipTableData::LegacyContactInfo(contact_info) => *contact_info.pubkey(),
            GossipTableData::Vote(_, vote) => vote.from,
            GossipTableData::LowestSlot(_, slots) => slots.from,
            GossipTableData::LegacySnapshotHashes(hash) => hash.from,
            GossipTableData::AccountsHashes(hash) => hash.from,
            GossipTableData::EpochSlots(_, p) => p.from,
            GossipTableData::LegacyVersion(version) => version.from,
            GossipTableData::Version(version) => version.from,
            GossipTableData::NodeInstance(node) => node.from,
            GossipTableData::DuplicateShred(_, shred) => shred.from,
            GossipTableData::SnapshotHashes(hash) => hash.from,
            GossipTableData::ContactInfo(node) => *node.pubkey(),
            GossipTableData::RestartLastVotedForkSlots(slots) => slots.from,
            GossipTableData::RestartHeaviestFork(fork) => fork.from,
        }
    }
}

/// Implement the trait Signable to allow the GossipTableValue to be signed.
impl Signable for GossipTableValue {
    fn pubkey(&self) -> Pubkey {
        self.pubkey()
    }

    fn signable_data(&self) -> Cow<[u8]> {
        Cow::Owned(serialize(&self.data).expect("failed to serialize CrdsData"))
    }

    fn get_signature(&self) -> Signature {
        self.signature
    }

    fn set_signature(&mut self, signature: Signature) {
        self.signature = signature
    }

    fn verify(&self) -> bool {
        self.get_signature()
            .verify(self.pubkey().as_ref(), self.signable_data().borrow())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GossipTableData {
    LegacyContactInfo(LegacyContactInfo),
    Vote(VoteIndex, Vote),
    LowestSlot(u8, LowestSlot),
    LegacySnapshotHashes(LegacySnapshotHashes),
    AccountsHashes(AccountsHashes),
    EpochSlots(EpochSlotsIndex, EpochSlots),
    LegacyVersion(LegacyVersion),
    Version(Version),
    NodeInstance(NodeInstance),
    DuplicateShred(DuplicateShredIndex, DuplicateShred),
    SnapshotHashes(SnapshotHashes),
    ContactInfo(ContactInfo),
    RestartLastVotedForkSlots(RestartLastVotedForkSlots),
    RestartHeaviestFork(RestartHeaviestFork),
}

type VoteIndex = u8;
type EpochSlotsIndex = u8;

type LegacySnapshotHashes = AccountsHashes;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountsHashes {
    pub from: Pubkey,
    pub hashes: Vec<(Slot, Hash)>,
    pub wallclock: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    pub from: Pubkey,
    transaction: Transaction,
    pub wallclock: u64,
    slot: Option<Slot>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LowestSlot {
    pub from: Pubkey,
    root: Slot,
    pub lowest: Slot,
    slots: BTreeSet<Slot>,
    stash: Vec<EpochIncompleteSlots>,
    pub wallclock: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EpochSlots {
    pub from: Pubkey,
    pub slots: Vec<CompressedSlots>,
    pub wallclock: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CompressedSlots {
    Flate2(Flate2),
    Uncompressed(Uncompressed),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Flate2 {
    pub first_slot: Slot,
    pub num: usize,
    pub compressed: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Uncompressed {
    pub first_slot: Slot,
    pub num: usize,
    pub slots: BitVec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LegacyVersion {
    pub from: Pubkey,
    pub wallclock: u64,
    pub version: solana_version::LegacyVersion1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Version {
    pub from: Pubkey,
    pub wallclock: u64,
    pub version: solana_version::LegacyVersion2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeInstance {
    from: Pubkey,
    wallclock: u64,
    timestamp: u64,
    token: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SnapshotHashes {
    pub from: Pubkey,
    pub full: (Slot, Hash),
    pub incremental: Vec<(Slot, Hash)>,
    pub wallclock: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestartHeaviestFork {
    pub from: Pubkey,
    pub wallclock: u64,
    pub last_slot: Slot,
    pub last_slot_hash: Hash,
    pub observed_stake: u64,
    pub shred_version: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct EpochIncompleteSlots {
    first: Slot,
    compression: CompressionType,
    compressed_list: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum CompressionType {
    Uncompressed,
    GZip,
    BZip2,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestartLastVotedForkSlots {
    pub from: Pubkey,
    pub wallclock: u64,
    offsets: SlotsOffsets,
    pub last_voted_slot: Slot,
    pub last_voted_hash: Hash,
    pub shred_version: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum SlotsOffsets {
    RunLengthEncoding(RunLengthEncoding),
    RawOffsets(RawOffsets),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct RunLengthEncoding(Vec<u16>);

#[derive(Deserialize, Serialize, Clone, Debug)]
struct RawOffsets(BitVec<u8>);

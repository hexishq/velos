use std::borrow::{Borrow, Cow};

use bincode::serialize;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Signable, Signature},
};

/// Gossip Table Value is replicated accross the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GossipTableValue {
    pub signature: Signature,
    pub data: GossipTableData,
}

/// Implement the trait Signable to allow the GossipTableValue to be signed.
impl Signable for GossipTableValue {
    #[warn(unconditional_recursion)]
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

/// GossipTableData that defines the different types of items GossipTableValues can hold.
///? Merge Strategy - Latest wallclock is picked
///? LowestSlot index is deprecated
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GossipTableData {}

use {
    crate::{
        data::GossipTableValue,
        filter::DataFilter,
        ping_pong::{Ping, Pong},
    },
    serde::{Deserialize, Serialize},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Deserialize, Serialize)]
pub enum Protocol {
    PullRequest(DataFilter, GossipTableValue),
    PullResponse(Pubkey, Vec<GossipTableValue>),
    PushMessage,
    PruneMessage,
    PingMessage(Ping),
    PongMessage(Pong),
}

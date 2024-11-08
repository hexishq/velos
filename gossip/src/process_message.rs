use {
    crate::protocol::Protocol,
    bincode::{deserialize, Error},
    std::net::SocketAddr,
};

pub fn process_message(message: Vec<u8>, from: SocketAddr) -> Result<(), Error> {
    let protocol: Protocol = deserialize(&message)?;

    match protocol {
        Protocol::PullRequest(data_filter, gossip_table_value) => (),
        Protocol::PullResponse(pubkey, vec) => (),
        Protocol::PushMessage => (),
        Protocol::PruneMessage => (),
        Protocol::PingMessage(ping) => todo!(),
        Protocol::PongMessage(pong) => todo!(),
    };

    Ok(())
}

use {
    crate::{
        contact_info::ContactInfo,
        data::{GossipTableData, GossipTableValue},
        filter::DataFilter,
        protocol::Protocol,
    },
    bincode::serialize,
    solana_sdk::signature::Keypair,
    thiserror::Error,
};

fn create_pull_request_message(
    contact_info: ContactInfo,
    filter: DataFilter,
    keypair: &Keypair,
) -> Result<Vec<u8>, PushMessagesErrors> {
    if contact_info.sockets().is_empty() {
        return Err(PushMessagesErrors::NoSocketEntry);
    }

    let signed_data =
        GossipTableValue::new_signed(GossipTableData::ContactInfo(contact_info), keypair);

    let protocol = Protocol::PullRequest(filter, signed_data);

    let message = match serialize(&protocol) {
        Ok(v) => v,
        Err(_) => return Err(PushMessagesErrors::SerializeFailed),
    };

    Ok(message)
}

#[derive(Debug, Error)]
enum PushMessagesErrors {
    #[error("No socket adress in contact info")]
    NoSocketEntry,
    #[error("Failed to serialize message")]
    SerializeFailed,
}

#[cfg(test)]
mod tests {

    use {
        super::*,
        crate::connection::{get_env_vars, Connection},
        bincode::deserialize,
        solana_sdk::{pubkey::Pubkey, signer::Signer, timing::timestamp},
        std::{net::SocketAddr, time::Duration},
        tokio::time::timeout,
    };

    fn process_pull_response(message: Vec<u8>) -> Option<(Pubkey, Vec<GossipTableValue>)> {
        let protocol: Protocol = deserialize(&message).ok()?;

        match protocol {
            Protocol::PullResponse(pubkey, vec) => Some((pubkey, vec)),
            _ => None,
        }
    }

    #[tokio::test]
    async fn test_send_pull_request() {
        let (udp, _, gossip_addr, peer_node_addr) = get_env_vars();
        let connection = Connection::new(&udp)
            .await
            .expect("Failed to create connection");

        let peer_node: SocketAddr = peer_node_addr
            .parse()
            .expect("Failed create node peer socket");
        let gossip_socket: SocketAddr = gossip_addr.parse().expect("Failed create gossip socket");

        connection.start_sending();
        connection.start_receiving();

        let keypair = Keypair::new();

        let contact_info = ContactInfo::new(keypair.pubkey(), timestamp(), 0, gossip_socket);

        let filter = DataFilter::default();

        let message = match create_pull_request_message(contact_info, filter, &keypair) {
            Ok(m) => m,
            Err(e) => {
                panic!("Failed to create pull request message {}", e)
            }
        };

        if let Err(e) = connection.tx_out.send((message, peer_node)).await {
            panic!("Failed to send message: {:?}", e);
        }

        let result = timeout(Duration::from_secs(20), async {
            loop {
                let msg_opt = {
                    let mut listen_channel = connection.rx_in.lock().await;
                    listen_channel.recv().await
                };
                if let Some((msg, from)) = msg_opt {
                    if from == peer_node {
                        let response = process_pull_response(msg);
                        match response {
                            Some(_) => return Ok(()),
                            None => return Err("Failed to process pull response"),
                        }
                    }
                }
            }
        })
        .await;

        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}

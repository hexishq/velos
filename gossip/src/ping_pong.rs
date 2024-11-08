use {
    bincode::{serialize, Error},
    lru::LruCache,
    rand::Rng,
    serde::{Deserialize, Serialize},
    solana_sdk::{
        hash::{self, Hash},
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    std::{
        net::SocketAddr,
        num::NonZero,
        time::{Duration, Instant},
    },
};

const GOSSIP_PING_TOKEN_SIZE: usize = 32;
const PING_PONG_HASH_PREFIX: &[u8] = "SOLANA_PING_PONG".as_bytes();

#[derive(Debug, Deserialize, Serialize)]
pub struct Pong {
    from: Pubkey,
    hash: Hash,
    signature: Signature,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ping {
    from: Pubkey,
    token: [u8; GOSSIP_PING_TOKEN_SIZE],
    signature: Signature,
}

impl Ping {
    fn new(token: [u8; GOSSIP_PING_TOKEN_SIZE], keypair: &Keypair) -> Result<Self, Error> {
        let signature = keypair.sign_message(&serialize(&token)?);
        let ping = Ping {
            from: keypair.pubkey(),
            token,
            signature,
        };
        Ok(ping)
    }

    pub fn rand(keypair: &Keypair) -> Result<Self, Error> {
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 32] = rng.gen();

        Self::new(random_bytes, keypair)
    }
}

impl Pong {
    pub fn new<T: Serialize>(ping: &Ping, keypair: &Keypair) -> Result<Self, Error> {
        let token = serialize(&ping.token)?;
        let hash = hash::hashv(&[PING_PONG_HASH_PREFIX, &token]);
        let pong = Pong {
            from: keypair.pubkey(),
            hash,
            signature: keypair.sign_message(hash.as_ref()),
        };
        Ok(pong)
    }

    pub fn from(&self) -> &Pubkey {
        &self.from
    }
}

pub struct PingCache {
    ttl: Duration,
    rate_limit_delay: Duration,
    pings: LruCache<(Pubkey, SocketAddr), Instant>,
    pongs: LruCache<(Pubkey, SocketAddr), Instant>,
    pending_cache: LruCache<Hash, (Pubkey, SocketAddr)>,
}

impl PingCache {
    pub fn new(ttl: Duration, rate_limit_delay: Duration, cap: NonZero<usize>) -> Self {
        Self {
            ttl,
            rate_limit_delay,
            pings: LruCache::new(cap),
            pongs: LruCache::new(cap),
            pending_cache: LruCache::new(cap),
        }
    }

    pub fn add(&mut self, pong: &Pong, socket: SocketAddr, now: Instant) -> bool {
        let node = (pong.from, socket);
        match self.pending_cache.peek(&pong.hash) {
            Some(value) if *value == node => {
                self.pings.pop(&node);
                self.pongs.put(node, now);
                self.pending_cache.pop(&pong.hash);
                true
            }
            _ => false,
        }
    }

    fn maybe_ping<F>(
        &mut self,
        now: Instant,
        node: (Pubkey, SocketAddr),
        mut pingf: F,
    ) -> Option<Ping>
    where
        F: FnMut() -> Option<Ping>,
    {
        match self.pings.peek(&node) {
            Some(t) if now.saturating_duration_since(*t) < self.rate_limit_delay => None,
            _ => {
                let ping = pingf()?;
                let token = serialize(&ping.token).ok()?;
                let hash = hash::hashv(&[PING_PONG_HASH_PREFIX, &token]);
                self.pending_cache.put(hash, node);
                self.pings.put(node, now);
                Some(ping)
            }
        }
    }

    pub fn check<F>(
        &mut self,
        now: Instant,
        node: (Pubkey, SocketAddr),
        pingf: F,
    ) -> (bool, Option<Ping>)
    where
        F: FnMut() -> Option<Ping>,
    {
        let (check, should_ping) = match self.pongs.get(&node) {
            None => (false, true),
            Some(t) => {
                let age = now.saturating_duration_since(*t);
                if age > self.ttl {
                    self.pongs.pop(&node);
                }
                (true, age > self.ttl / 8)
            }
        };
        let ping = if should_ping {
            self.maybe_ping(now, node, pingf)
        } else {
            None
        };
        (check, ping)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            connection::{get_env_vars, Connection},
            protocol::Protocol,
        },
        bincode::deserialize,
        tokio::{self, time::timeout},
    };

    #[tokio::test]
    async fn test_send_ping() {
        //if you are using the mainnet or testnet maybe the test will fail
        //only because the entry point dont respond,
        //if the test fail you should try again some times to ensure the test really is working or just use the devnet
        let (udp, entry_point, _, _) = get_env_vars();
        let connection = Connection::new(&udp)
            .await
            .expect("Failed to create connection");

        let solana_entrypoint: SocketAddr = entry_point
            .parse()
            .expect("Failed create entrypoint socket");

        connection.start_sending();
        connection.start_receiving();

        let keypair = Keypair::new();
        let ping = Ping::rand(&keypair).expect("Failed to create ping");

        let message = serialize(&Protocol::PingMessage(ping)).expect("Failed to serealize ping");

        if let Err(e) = connection.tx_out.send((message, solana_entrypoint)).await {
            panic!("Failed to send message: {:?}", e);
        }

        let result = timeout(Duration::from_secs(10), async {
            loop {
                let msg_opt = {
                    let mut listen_channel = connection.rx_in.lock().await;
                    listen_channel.recv().await
                };
                if let Some((msg, from)) = msg_opt {
                    if from == solana_entrypoint {
                        let protocol: Protocol =
                            deserialize(&msg).expect("Failed to deserialize message");
                        match protocol {
                            Protocol::PongMessage(_pong) => {
                                return Ok(());
                            }
                            _ => {
                                return Err("Received a message that is not a Pong");
                            }
                        }
                    }
                }
            }
        })
        .await;

        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}

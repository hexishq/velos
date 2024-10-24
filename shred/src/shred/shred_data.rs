use crate::shred::{legacy, merkle};

pub enum ShredData {
    Legacy(legacy::ShredData),
    Merkle(merkle::ShredData),
}

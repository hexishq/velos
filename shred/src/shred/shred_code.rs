use crate::shred::{legacy, merkle};

pub enum ShredCode {
    Legacy(legacy::ShredCode),
    Merkle(merkle::ShredCode),
}

use {
    serde::{Deserialize, Serialize},
    solana_bloom::bloom::Bloom,
    solana_sdk::hash::Hash,
};

const MASK_BITS: u32 = 7427;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataFilter {
    pub filter: Bloom<Hash>,
    mask: u64,
    mask_bits: u32,
}

impl DataFilter {
    pub fn mask_bits(&self) -> u32 {
        self.mask_bits
    }
}

impl Default for DataFilter {
    fn default() -> Self {
        fn compute_mask(seed: u64, mask_bits: u32) -> u64 {
            assert!(seed <= 2u64.pow(mask_bits));
            let seed: u64 = seed.checked_shl(64 - mask_bits).unwrap_or(0x0);
            seed | (!0u64).checked_shr(mask_bits).unwrap_or(!0x0)
        }

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        fn mask_bits(num_items: f64, max_items: f64) -> u32 {
            // for small ratios this can result in a negative number, ensure it returns 0 instead
            ((num_items / max_items).log2().ceil()).max(0.0) as u32
        }

        let max_items: u32 = 1287;
        let num_items: u32 = 0;
        let false_rate: f64 = 0.1f64;
        let max_bits = MASK_BITS;
        let mask_bits = mask_bits(f64::from(num_items), f64::from(max_items));

        let bloom: Bloom<Hash> = Bloom::random(num_items as usize, false_rate, max_bits as usize);

        DataFilter {
            filter: bloom,
            mask: compute_mask(0_u64, mask_bits),
            mask_bits,
        }
    }
}

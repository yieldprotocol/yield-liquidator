pub mod bindings;
pub mod borrowers;
pub mod escalator;
pub mod keeper;
pub mod liquidations;

use ethers::prelude::*;
use std::collections::HashMap;

/// "ETH-A" collateral type in hex, right padded to 32 bytes
pub const WETH: [u8; 32] = [
    0x45, 0x54, 0x48, 0x2d, 0x41, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
];

// merge & deduplicate the 2 data structs
pub fn merge<T>(a: Vec<Address>, b: &HashMap<Address, T>) -> Vec<Address> {
    let keys = b.keys().cloned().collect::<Vec<_>>();
    let mut all = [a, keys].concat();
    all.sort_unstable();
    all.dedup();
    all
}

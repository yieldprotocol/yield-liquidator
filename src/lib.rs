pub mod auctions;
pub mod bindings;
pub mod positions;

/// "ETH-A" collateral type in hex, right padded to 32 bytes
pub const WETH: [u8; 32] = [
    0x45, 0x54, 0x48, 0x2d, 0x41, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
];

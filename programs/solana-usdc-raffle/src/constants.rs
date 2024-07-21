use anchor_lang::prelude::Pubkey;

pub const POOL_SEED: &str = "pool";
pub const POOL_NATIVE_SEED: &str = "pool_native";

pub const MAX_BUYER_COUNT: usize = 200;
pub const MAX_TOTAL_TICKET: u32 = 100;
pub const MAX_RAFFLE_ID_LEN: usize = 50;
pub const DECIMAL: u64 = 100000000; // 8 for BPT

pub const ADMIN_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("CxMudY9Vyw4p5fx1ZY173GHH2Q1ewZFo2YWmd8sozquQ"); 
pub const ADMIN_ADDRESS_SECOND: Pubkey = anchor_lang::solana_program::pubkey!("CxMudY9Vyw4p5fx1ZY173GHH2Q1ewZFo2YWmd8sozquQ"); 
pub const TREASURY_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("6CEUN4oMGbCQjNrzACaTKPuQKgCmqytgCDbtq5L4r6Em"); 
// pub const PAY_TOKEN_MINT_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"); // USDC address for mainnet
// pub const PAY_TOKEN_MINT_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("HSvEJfU8hXUWFRodbVbRfwYb2p4DwSwpiMaoB7UDRVD4"); // USDT address for devnet
pub const PAY_TOKEN_MINT_ADDRESS: Pubkey = anchor_lang::solana_program::pubkey!("7FctSfSZ9GonfMrybp45hzoQyU71CEjjZFxxoSzqKWT"); // BPT address for devnet
pub const SOL_KEY: Pubkey = anchor_lang::solana_program::pubkey!("So11111111111111111111111111111111111111112");





use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};

use instructions::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("Emh7UKzAX4BZY8tF7fmPecQ2p56DKYkaND19MfNf7FX5");

#[program]
pub mod solana_usdc_raffle {
    use super::*;

    pub fn create_raffle(
        ctx: Context<CreateRaffleContext>,
        raffle_id: String,
        start_time: u32,
        reserved: f32,
        price: u32,
        prize: u32,
        auto_generate: u8,
        multiplier: f32,
        account_fee: u32
    ) -> Result<()> {
        instructions::create_raffle::create_raffle(
            ctx, 
            raffle_id, 
            start_time,  
            reserved, 
            price, 
            prize,
            auto_generate,
            multiplier,
            account_fee
        )
    }

    pub fn buy_tickets(
        ctx: Context<BuyTicketContext>, 
        total_ticket: u32, 
        total_price: u64, 
        account_fee_sol: u32, 
        force: [u8; 32]
    ) -> Result<()> {
        instructions::buy_tickets::buy_tickets(ctx, total_ticket, total_price, account_fee_sol, force)
    }

    pub fn complete_raffle(ctx: Context<CompleteRaffleContext>) -> Result<()> {
        // instructions::complete_raffle::complete_raffle(ctx)
        Ok(())
    }

    pub fn set_winner(ctx: Context<SetWinnerContext>, force: [u8; 32]) -> Result<()> {
        instructions::set_winner::set_winner(ctx,force)
    }

    pub fn claim_prize(ctx: Context<ClaimPrizeContext>, pool_bump: u8, pool_native_bump: u8) -> Result<()> {
        instructions::claim_prize::claim_prize(ctx, pool_bump, pool_native_bump)
    }
}


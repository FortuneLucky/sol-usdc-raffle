use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};
use std::mem::size_of;

use crate::state::pool::*;
use crate::state::buyers::*;
use crate::constants::{
  POOL_SEED,
  POOL_NATIVE_SEED,
  ADMIN_ADDRESS,
  MAX_TOTAL_TICKET
};
use crate::errors::{
  RaffleError
};


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
    msg!("Creating a new raffle with the following Id: {}", raffle_id);

    let a_pool = &mut ctx.accounts.pool;
    let buyers = &mut ctx.accounts.buyers.load_init()?;

    a_pool.raffle_id = raffle_id;
    a_pool.start_time = start_time;
    a_pool.ticket_price = price;
    a_pool.prize = prize;
    a_pool.new_random_address = a_pool.key();
    a_pool.reserved = reserved;
    a_pool.native_account = ctx.accounts.pool_native_account.key();
    a_pool.auto_generate = auto_generate;
    a_pool.multiplier = multiplier;
    a_pool.status = RaffleStatus::Active;
    a_pool.purchased_ticket = 0;
    a_pool.account_fee = account_fee;
    a_pool.buyers = ctx.accounts.buyers.key();
    a_pool.total_ticket = ((prize / price) as f32 + (prize / price) as f32 * reserved).ceil() as u32;

    msg!("New raffle created successfully");
    Ok(())
}

#[derive(Accounts)]
#[instruction(raffle_id: String)]
pub struct CreateRaffleContext<'info> {
  #[account(mut, constraint = admin.key() == ADMIN_ADDRESS)]
  pub admin: Signer<'info>,

  #[account(init, seeds = [
    POOL_SEED.as_bytes(), 
    &raffle_id.as_bytes()], 
    bump, 
    payer = admin, 
    space = size_of::<Pool>() + 8,
  )]
  pub pool: Account<'info, Pool>,

  /// CHECK: AccountInfo is an unchecked account, any account can be passed in
  #[account(mut, seeds = [
    POOL_NATIVE_SEED.as_bytes(), 
    &raffle_id.as_bytes()], 
    bump, 
    // payer = admin, 
    // space = 120
  )]
  pub pool_native_account: AccountInfo<'info>,

  #[account(zero)]
  pub buyers: AccountLoader<'info, Buyers>,
  
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>
}


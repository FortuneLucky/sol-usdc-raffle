use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  associated_token::get_associated_token_address,
  token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};
use anchor_lang::system_program::{ Transfer, transfer };

use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::NetworkState;
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

use crate::state::pool::*;
use crate::state::buyers::*;
use crate::constants::{
    POOL_SEED,
    POOL_NATIVE_SEED,
    ADMIN_ADDRESS,
    TREASURY_ADDRESS,
    PAY_TOKEN_MINT_ADDRESS,
    MAX_TOTAL_TICKET,
    DECIMAL,
};
use crate::utils::{
  get_account_data,
  current_state
};
use crate::errors::{
    RaffleError
};


pub fn claim_prize(ctx: Context<ClaimPrizeContext>, pool_bump: u8, pool_native_bump: u8) -> Result<()> {
  // Get the raffle pool info
  let a_pool = &mut ctx.accounts.pool;

  // Check if the given rafffle was completed
  require!(a_pool.status == RaffleStatus::Completed, RaffleError::NotFinishRaffle);

  // Get the buyers info for the raffle
  let buyers = &mut ctx.accounts.buyers.load_mut()?;


  // Check if the admin is correct
  require_keys_eq!(ctx.accounts.admin.key(), ADMIN_ADDRESS, RaffleError::InvalidAdmin);

  msg!("Claiming prize!");

  let admin_destination = &ctx.accounts.admin_ata;
  let destination = &ctx.accounts.winner_ata;
  let source = &ctx.accounts.pool_ata;
  let token_program = &ctx.accounts.token_program;

  let total_balance = source.amount;
  let destination_amount: u64 = a_pool.prize as u64 * u64::pow(10, 8);
  let admin_amount: u64 = total_balance - destination_amount;

  msg!("Total balance on the pool: {} and prize {}", total_balance, destination_amount);

  // Transfer prize amount among total amount on the pool to winner and rest to admin
  token::transfer(
    CpiContext::new_with_signer(
        token_program.to_account_info(),
        SplTransfer {
          from: source.to_account_info().clone(),
          to: destination.to_account_info().clone(),
          authority: a_pool.to_account_info().clone(),
        },
        &[&[POOL_SEED.as_bytes(), &a_pool.raffle_id.as_bytes(), &[pool_bump]]],
    ),
    destination_amount
  )?;

  token::transfer(
    CpiContext::new_with_signer(
        token_program.to_account_info(),
        SplTransfer {
          from: source.to_account_info().clone(),
          to: admin_destination.to_account_info().clone(),
          authority: a_pool.to_account_info().clone(),
        },
        &[&[POOL_SEED.as_bytes(), &a_pool.raffle_id.as_bytes(), &[pool_bump]]],
    ),
    admin_amount
  )?;

  msg!("pool sol amount {}", ctx.accounts.pool_native_account.to_account_info().lamports());

  // Transfer SOL in the pool to the admin wallet
  transfer(CpiContext::new_with_signer(
      ctx.accounts.system_program.to_account_info(), 
      Transfer {
          from: ctx.accounts.pool_native_account.to_account_info(),
          to: ctx.accounts.admin.to_account_info(),
      },
      &[&[POOL_NATIVE_SEED.as_bytes(), &a_pool.raffle_id.as_bytes(), &[pool_native_bump]]],
    ), 
    ctx.accounts.pool_native_account.to_account_info().lamports()
  )?;

  msg!("pool sol amount {}", ctx.accounts.pool_native_account.to_account_info().lamports());


  if a_pool.auto_generate == 1 {
    let current_timestamp = Clock::get()?.unix_timestamp as u32;

    a_pool.start_time = current_timestamp;
    a_pool.prize = (a_pool.prize as f32 * a_pool.multiplier).ceil() as u32;
    a_pool.status = RaffleStatus::Active;
    a_pool.purchased_ticket = 0;
    a_pool.total_ticket = (((a_pool.prize  / a_pool.ticket_price) as f32 + (a_pool.prize  / a_pool.ticket_price) as f32 * a_pool.reserved)).ceil() as u32;

    let (new_random_address, _random_bump_seed) = Pubkey::find_program_address( 
      &[
        a_pool.raffle_id.as_ref(),
        a_pool.start_time.to_be_bytes().as_ref(),
      ], 
      ctx.program_id
    );

    a_pool.new_random_address = new_random_address;
    
    buyers.count = 0;
  }
  Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrizeContext<'info> {
  #[account(mut)]
  pub pool: Account<'info, Pool>,
  
  /// CHECK: AccountInfo is an unchecked account, any account can be passed in
  #[account(mut)]
  pub pool_native_account: AccountInfo<'info>,

  #[account(mut)]
  pub buyers: AccountLoader<'info, Buyers>,

  pub pay_token_mint: Account<'info, Mint>,
  
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut, constraint = admin.key() == ADMIN_ADDRESS)]
  pub admin: SystemAccount<'info>,

  #[account(mut)]
  pub winner_ata: Account<'info, TokenAccount>,

  #[account(
    mut,
    associated_token::mint = pay_token_mint,
    associated_token::authority = ADMIN_ADDRESS,
  )]
  pub admin_ata: Account<'info, TokenAccount>,

  #[account(
    mut, 
    associated_token::mint = pay_token_mint,
    associated_token::authority = pool,
  )]
  pub pool_ata: Account<'info, TokenAccount>,

  token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  system_program: Program<'info, System>
}

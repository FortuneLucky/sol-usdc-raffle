use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};

use crate::state::pool::*;
use crate::state::buyers::*;
use crate::constants::{
    POOL_SEED,
    ADMIN_ADDRESS,
    TREASURY_ADDRESS,
    PAY_TOKEN_MINT_ADDRESS,
    MAX_TOTAL_TICKET,
    DECIMAL,
};
use crate::errors::{
    RaffleError
};


pub fn complete_raffle(ctx: Context<CompleteRaffleContext>) -> Result<()> {
  // Get the raffle pool info
  let a_pool = &mut ctx.accounts.pool;

  // Check if the given rafffle was still Active
  require!(a_pool.status == RaffleStatus::Completed, RaffleError::NotFinishRaffle);

  // Check if the signer is a correct winner
  require_keys_eq!(ctx.accounts.winner.key(), a_pool.winner, RaffleError::SetWinnerError);

  msg!("Claiming prize!");

  // Accounts from context
  let a_winner = &ctx.accounts.winner;
  let admin_destination = &ctx.accounts.admin_ata;
  let destination = &ctx.accounts.winner_ata;
  let source = &ctx.accounts.treasury_ata;
  let token_program = &ctx.accounts.token_program;
  let authority = &ctx.accounts.admin;

  let total_price = ((a_pool.purchased_ticket as u64 * a_pool.ticket_price as u64) as f64 * 0.9).ceil() as u64;
  let destination_amount: u64 = a_pool.prize.into();
  let admin_amount: u64 = total_price - a_pool.prize as u64;

  // Transfer paytoken to both admin(5% if there is referral, otherwise 10%) and treasury(90%)
  token::transfer(
    CpiContext::new(
        token_program.to_account_info(),
        SplTransfer {
          from: source.to_account_info().clone(),
          to: destination.to_account_info().clone(),
          authority: authority.to_account_info().clone(),
        },
    ),
    destination_amount,
  )?;

  token::transfer(
    CpiContext::new(
        token_program.to_account_info(),
        SplTransfer {
          from: source.to_account_info().clone(),
          to: admin_destination.to_account_info().clone(),
          authority: authority.to_account_info().clone(),
        },
    ),
    admin_amount,
  )?;

  Ok(())
}

#[derive(Accounts)]
pub struct CompleteRaffleContext<'info> {
  #[account(mut)]
  // pub pool: Box<Account<'info, Pool>>,
  pub pool: Account<'info, Pool>,

  pub pay_token_mint: Account<'info, Mint>,

  #[account(mut)]
  pub admin: Signer<'info>,

  pub winner: SystemAccount<'info>,

  #[account(
    mut, 
    associated_token::mint = pay_token_mint,
    associated_token::authority = winner,
  )]
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
    associated_token::authority = TREASURY_ADDRESS,
  )]
  pub treasury_ata: Account<'info, TokenAccount>,

  token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  system_program: Program<'info, System>
}

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


pub fn set_winner(ctx: Context<SetWinnerContext>, force: [u8; 32]) -> Result<()> {
  // Get the raffle pool info
  let a_pool = &mut ctx.accounts.pool;

  // Check if the given rafffle was still Active
  require!(a_pool.status == RaffleStatus::Processing, RaffleError::NoVRFRequest);

  // Get the buyers info for the raffle
  let buyers = &mut ctx.accounts.buyers.load_mut()?;

  // Get random number result from orao vrf
  let rand_acc = get_account_data(&ctx.accounts.random)?;

  let randomness = current_state(&rand_acc);

  require!(randomness != 0, RaffleError::StillProcessing);
 
  let result = randomness % a_pool.purchased_ticket as u64;


  // // Check if the signer is a correct winner
  // require_keys_eq!(ctx.accounts.winner.key(), a_pool.winner, RaffleError::SetWinnerError);

  // Check if the admin is correct
  require_keys_eq!(ctx.accounts.admin.key(), ADMIN_ADDRESS, RaffleError::InvalidAdmin);

  msg!("Setting winner!");

  // Accounts from context
  let winner_index = buyers.set_winner(result);
  let a_winner = buyers.buyers[winner_index].buyer;
  msg!("Setting winner result {}, {}, {}, {}, {}!", a_pool.purchased_ticket, randomness, result, winner_index, a_winner);

  a_pool.winner = a_winner;
  a_pool.winner_ticket_number = result as u32;
  a_pool.status = RaffleStatus::Completed;
  
  Ok(())
}

#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct SetWinnerContext<'info> {
  #[account(mut)]
  pub pool: Account<'info, Pool>,

  #[account(mut)]
  pub buyers: AccountLoader<'info, Buyers>,
  
  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(mut, constraint = admin.key() == ADMIN_ADDRESS)]
  pub admin: SystemAccount<'info>,

  /// CHECK: Randomness
  #[account(
      mut,
      // seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force],
      // bump,
      // seeds::program = orao_solana_vrf::ID
  )]
  pub random: AccountInfo<'info>,

  #[account(
      mut,
      seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
      bump,
      seeds::program = orao_solana_vrf::ID
  )]
  pub config: Account<'info, NetworkState>,

  pub vrf: Program<'info, OraoVrf>,

  token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  system_program: Program<'info, System>
}

use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
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
    ADMIN_ADDRESS,
    TREASURY_ADDRESS,
    PAY_TOKEN_MINT_ADDRESS,
    MAX_TOTAL_TICKET,
    DECIMAL,
};
use crate::utils::*;
use crate::errors::{
    RaffleError
};


pub fn buy_tickets(ctx: Context<BuyTicketContext>, total_ticket: u32, total_price: u64, account_fee_sol: u32, force: [u8; 32]) -> Result<()> {
    // Get the raffle pool info
    let a_pool = &mut ctx.accounts.pool;

    let buyers = &mut ctx.accounts.buyers.load_mut()?;

    // Check if the given rafffle was already finished
    require!(a_pool.status == RaffleStatus::Active, RaffleError::AlreadyFinished);
    
    // Check if the account fee sol amount is correct
    msg!("Compare! {}, {}, {}, {}, {}", a_pool.account_fee as u64 * total_ticket as u64 / a_pool.total_ticket as u64 == account_fee_sol as u64, a_pool.account_fee, total_ticket, a_pool.total_ticket, account_fee_sol);
    require!(a_pool.account_fee as u64 * total_ticket as u64 / a_pool.total_ticket as u64 == account_fee_sol as u64, RaffleError::InvalidAccountFeeAmount);

    // Display total tickets to purchase along with its total price
    msg!("Buying {} tickets with {}", total_ticket, total_price / DECIMAL as u64);

    // Check if the total ticket is not zero
    require!(total_ticket > 0, RaffleError::InvalidAmount);
    
    // Check if the total ticket is less than or equal to remaining tickets
    require!(total_ticket <= a_pool.total_ticket - a_pool.purchased_ticket, RaffleError::TooManyTicket);

    // Check if payment token is correct
    require_keys_eq!(ctx.accounts.pay_token_mint.key(), PAY_TOKEN_MINT_ADDRESS, RaffleError::PayTokenMintAddressError);

    // Check if referrer provided or not
    if let Some(referral) = &ctx.accounts.referral {
      require_keys_neq!(ctx.accounts.buyer.key(), referral.key(), RaffleError::ReferralError);

      if let Some(referral_ata) = &ctx.accounts.referral_ata {
        msg!("referral ata provided!");
      } else {
        require!(false, RaffleError::ReferralAtaError);
      }
    }

    // Accounts from context
    let a_buyer = &ctx.accounts.buyer;
    let destination = &ctx.accounts.admin_ata;
    let source = &ctx.accounts.buyer_ata;
    let pool_ata = &ctx.accounts.pool_ata;
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.buyer;

    let mut admin_amount: u64 = (total_price / 100 * 10).try_into().unwrap();
    let pool_amount: u64 = (total_price / 100 * 90).try_into().unwrap();

    // Check if a referral was provided
    if let Some(referral) = &ctx.accounts.referral {
      msg!("Referral provided: {}", referral.key());

      let referral_amount: u64 = (total_price / 100 * 5).try_into().unwrap();
      admin_amount = (total_price / 100 * 5).try_into().unwrap();

      if let Some(referral_ata) = &ctx.accounts.referral_ata {
        // Transfer 5% paytoken amount to referral
        token::transfer(
          CpiContext::new(
              token_program.to_account_info(),
              SplTransfer {
                from: source.to_account_info().clone(),
                to: referral_ata.to_account_info().clone(),
                authority: authority.to_account_info().clone(),
              },
          ),
          referral_amount,
        )?;
      } 
    }
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
      admin_amount,
    )?;

    token::transfer(
      CpiContext::new(
          token_program.to_account_info(),
          SplTransfer {
            from: source.to_account_info().clone(),
            to: pool_ata.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
          },
      ),
      pool_amount,
    )?;

    // Collect some amount of SOL to create a new account for the next raffle pool
    let cpi_context = CpiContext::new(
      ctx.accounts.system_program.to_account_info(), 
      Transfer {
          from: ctx.accounts.buyer.to_account_info().clone(),
          to: ctx.accounts.pool_native_account.to_account_info(),
      });
    
    transfer(cpi_context, account_fee_sol as u64)?;

    // Update the pool info based on the buyer's amount
    if let Some(receiver) = &ctx.accounts.receiver {
      buyers.buy_ticket(receiver.key(), total_ticket)?;
      a_pool.purchased_ticket += total_ticket;
    } else {
      buyers.buy_ticket(a_buyer.to_account_info().key(), total_ticket)?;
      a_pool.purchased_ticket += total_ticket;
    }    

    let random = &ctx.accounts.random;
    let config = &ctx.accounts.config;
    let vrf = &ctx.accounts.vrf;
    let treasury = &ctx.accounts.treasury;

    // if a_pool.purchased_ticket > 20 && random.is_some() && config.is_some() && vrf.is_some() && treasury.is_some() {// for just testing
    if a_pool.purchased_ticket * a_pool.ticket_price as u32 >= (a_pool.prize as f32 + a_pool.prize as f32 * a_pool.reserved).ceil() as u32 &&
      random.is_some() && 
      config.is_some() && 
      vrf.is_some() && 
      treasury.is_some() 
    {
      a_pool.status = RaffleStatus::Processing;
      a_pool.last_buyer = authority.key();

      // Request vrf using CPI
      let cpi_program = vrf.as_ref().unwrap().to_account_info();
      let cpi_accounts = orao_solana_vrf::cpi::accounts::Request {
          payer: ctx.accounts.buyer.to_account_info(),
          network_state: config.as_ref().unwrap().to_account_info(),
          treasury: treasury.as_ref().unwrap().to_account_info(),
          request: random.as_ref().unwrap().to_account_info(),
          system_program: ctx.accounts.system_program.to_account_info(),
      };
      let cpi_ctx = anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
      orao_solana_vrf::cpi::request(cpi_ctx, force)?;
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct BuyTicketContext<'info> {
  #[account(mut)]
  pub pool: Account<'info, Pool>,

  /// CHECK: AccountInfo is an unchecked account, any account can be passed in
  #[account(mut)]
  pub pool_native_account: AccountInfo<'info>,

  pub pay_token_mint: Account<'info, Mint>,

  #[account(mut)]
  pub buyer: Signer<'info>,

  #[account(mut)]
  pub buyers: AccountLoader<'info, Buyers>,

  // Referral is optional
  pub referral:  Option<SystemAccount<'info>>,

  // Receiver is optional
  pub receiver:  Option<SystemAccount<'info>>,

  #[account(
    mut, 
    associated_token::mint = pay_token_mint,
    associated_token::authority = buyer,
  )]
  pub buyer_ata: Account<'info, TokenAccount>,

 
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

  // Optional
  #[account(
    mut, 
    associated_token::mint = pay_token_mint,
    associated_token::authority = referral,
  )]
  pub referral_ata: Option<Account<'info, TokenAccount>>,
  
  // Oracle VRF related optional params
  /// CHECK: Treasury
  #[account(mut)]
  pub treasury: Option<AccountInfo<'info>>,
  
  /// CHECK: Randomness
  #[account(
      mut,
      // seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force],
      // bump,
      // seeds::program = orao_solana_vrf::ID
  )]
  pub random: Option<AccountInfo<'info>>,

  #[account(
      mut,
      seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
      bump,
      seeds::program = orao_solana_vrf::ID
  )]
  pub config: Option<Account<'info, NetworkState>>,

  pub vrf: Option<Program<'info, OraoVrf>>,

  token_program: Program<'info, Token>,
  associated_token_program: Program<'info, AssociatedToken>,
  system_program: Program<'info, System>
}

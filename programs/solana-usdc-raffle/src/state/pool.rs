use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::constants::*;
use crate::errors::{
  RaffleError
};

#[account]
pub struct Pool {
  pub buyers: Pubkey,
  pub raffle_id: String, 
  pub start_time: u32,
  pub ticket_price: u32,
  pub prize: u32,
  pub native_account: Pubkey,
  pub winner_ticket_number: u32,
  pub winner: Pubkey,
  pub new_random_address: Pubkey,
  pub last_buyer: Pubkey,
  pub reserved: f32,
  pub total_ticket: u32,
  pub purchased_ticket: u32,
  pub auto_generate: u8,
  pub multiplier: f32,
  pub status: RaffleStatus,
  pub account_fee: u32,
}

impl Pool {
    pub const LEN: usize = size_of::<Self>();
}

impl Default for Pool {
  #[inline]
  fn default() -> Pool {
      Pool {
        raffle_id: "".to_string(),
        start_time: 0,
        ticket_price: 0,
        prize: 0,
        winner_ticket_number: 0,
        winner: Pubkey::default(),
        new_random_address: Pubkey::default(),
        native_account: Pubkey::default(),
        last_buyer: Pubkey::default(),
        buyers: Pubkey::default(),
        reserved: 0.0,
        total_ticket: 0,
        purchased_ticket: 0,
        auto_generate: 0,
        multiplier: 1.1,
        status: RaffleStatus::Active,
        account_fee: 0,
      }
  }
}

#[derive(Eq, AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RaffleStatus {
    Active,
    Processing,
    Completed,
}

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock};
use anchor_lang::{
  prelude::{AccountInfo, CpiContext, Program, Result},
  system_program::{self, System, Transfer as SolanaTransfer},
  ToAccountInfo,
};
use orao_solana_vrf::state::Randomness;
use std::mem::size_of;
use crate::errors::{
  RaffleError
};

pub fn get_current_time() -> Result<u32> {
  let clock = clock::Clock::get().unwrap();
  Ok(clock.unix_timestamp as u32)
}

pub fn get_account_data(account_info: &AccountInfo) -> Result<Randomness> {
  require!(!account_info.data_is_empty(), RaffleError::UninitializedAccount);

  let account = Randomness::try_deserialize(&mut &account_info.data.borrow()[..])?;

  Ok(account)
}


pub fn current_state(randomness: &Randomness) ->u64 {
  if let Some(randomness) = randomness.fulfilled() {
      let value = randomness[0..size_of::<u64>()].try_into().unwrap();
      
      return u64::from_le_bytes(value);
  } else {
      return 0;
  }
}
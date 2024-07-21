use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::constants::*;
use crate::errors::{
    RaffleError
};

#[account(zero_copy)]
pub struct Buyers {
  pub buyers: [Buyer; MAX_BUYER_COUNT],
  pub count: u32,
}

impl Default for Buyers {
  #[inline]
  fn default() -> Buyers {
      Buyers {
        buyers: [
          Buyer {
              ..Default::default()
          }; MAX_BUYER_COUNT
        ],
        count: 0,
      }
  }
}

impl Buyers {
  fn find_buyer(&self, buyer: Pubkey) -> usize {
    let mut index = MAX_BUYER_COUNT;
    for i in 0..self.count as usize{
      if self.buyers[i].buyer == buyer {
        index = i;
        break;
      }
    }

    index
  }

  pub fn buy_ticket(&mut self, buyer: Pubkey, amount: u32) -> Result<()> {
    let index = self.find_buyer(buyer);
    msg!("index {}", index);
    if index == MAX_BUYER_COUNT {
      self.buyers[self.count as usize] = Buyer {
        buyer,
        purchased_ticket: amount,
      };
      self.count += 1;
      require!((self.count as usize) < MAX_BUYER_COUNT, RaffleError::OverMaxCount);
    }
    else {
      self.buyers[index].purchased_ticket += amount;
    }

    Ok(())
  }

  pub fn set_winner(&mut self, random: u64) -> usize {
    let mut start: u32 = 0;
    let mut winner: usize = 0;
    
    for i in 0..self.count as usize {
      msg!("start {}, random {}, {}, {}, {}, {}", start, random, self.buyers[i].purchased_ticket, random as u32 >= start, start + self.buyers[i].purchased_ticket > random as u32, random as u32 >= start && start + self.buyers[i].purchased_ticket > random as u32);
      if random as u32 >= start && start + self.buyers[i].purchased_ticket > random as u32 {
        winner = i;
        break;
      }
      start += self.buyers[i].purchased_ticket;
    }

    winner
  }
}

#[zero_copy]
#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct Buyer {
  pub buyer: Pubkey,
  pub purchased_ticket: u32,
}

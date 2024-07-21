use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{ self, Mint, Token, TokenAccount, Transfer as SplTransfer }
};

use crate::state::pool::*;
use crate::constants::{
    ADMIN_KEY,
    POOL_SEED,
    COMMUNITY_KEY,
    PAY_TOKEN,
    MAX_TOTAL_TICKET
};
use crate::errors::{
    RaffleError
};

// logic here
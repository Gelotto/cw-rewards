use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BlockInfo, Timestamp, Uint128};
use cw_lib::models::{Token, TokenAmount};

pub type RewardKey = (Addr, String); // (contract address, unique ident)
pub type GrantKey = (Addr, RewardKey); // (wallet address, RewardKey)

#[cw_serde]
pub enum Expiration {
  Time(Timestamp),
  Height(u64),
}

#[cw_serde]
pub struct RewardInitArgs {
  pub key: String,
  pub count: Option<u32>,
  pub message: Option<String>,
  pub expires_at: Option<Expiration>,
  pub tokens: Vec<TokenAmount>,
}

#[cw_serde]
pub struct RewardMetadata {
  pub claims_processed: u32,
  pub claims_remaining: Option<u32>,
  pub message: Option<String>,
  pub tokens: Vec<TokenAmount>,
  pub is_paused: bool,
}

#[cw_serde]
pub struct RewardQueryResult {
  pub key: String,
  pub sender: Addr,
  pub claims_processed: u32,
  pub claims_remaining: Option<u32>,
  pub message: Option<String>,
  pub tokens: Vec<TokenAmount>,
  pub is_paused: bool,
}

#[cw_serde]
pub struct Grant {
  pub expires_at: Option<Expiration>,
}

#[cw_serde]
pub struct GrantQueryResult {
  pub sender: Addr,
  pub key: String,
  pub claims_processed: u32,
  pub claims_remaining: Option<u32>,
  pub message: Option<String>,
}

#[cw_serde]
pub struct ClaimTotals {
  pub token: Token,
  pub amount_claimed: Uint128,
  pub amount_granted: Uint128,
}

impl Grant {
  pub fn is_expired(
    &self,
    current_block: &BlockInfo,
  ) -> bool {
    if let Some(expires_at) = &self.expires_at {
      match expires_at {
        Expiration::Time(t) => *t <= current_block.time,
        Expiration::Height(h) => *h <= current_block.height,
      }
    } else {
      false
    }
  }
}

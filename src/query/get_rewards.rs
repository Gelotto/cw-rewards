use std::marker::PhantomData;

use crate::{
  error::ContractError, models::RewardQueryResult, msg::RewardsResponse, state::REWARDS,
};
use cosmwasm_std::{Addr, Deps, Order};
use cw_storage_plus::Bound;

pub fn get_rewards(
  deps: Deps,
  contract_addr: &Addr,
  some_limit: Option<u8>,
  some_cursor: Option<String>,
) -> Result<RewardsResponse, ContractError> {
  let limit = some_limit.unwrap_or(20).clamp(1, 20) as usize;
  let start = (|| -> Result<_, ContractError> {
    if let Some(key) = some_cursor {
      Ok(Some(Bound::Inclusive((key, PhantomData))))
    } else {
      Err(ContractError::NotAuthorized {})
    }
  })()?;

  let mut rewards: Vec<RewardQueryResult> = vec![];

  for result in REWARDS
    .prefix(contract_addr.clone())
    .range(deps.storage, start, None, Order::Descending)
    .take(limit)
  {
    if let Ok((key, meta)) = result {
      rewards.push(RewardQueryResult {
        claims_processed: meta.claims_processed,
        claims_remaining: meta.claims_remaining,
        message: meta.message,
        tokens: meta.tokens.clone(),
        sender: contract_addr.clone(),
        is_paused: meta.is_paused,
        key,
      });
    }
  }

  let cursor = if !rewards.is_empty() {
    if let Some(last_reward) = rewards.last() {
      Some(last_reward.key.clone())
    } else {
      None
    }
  } else {
    None
  };

  Ok(RewardsResponse { rewards, cursor })
}

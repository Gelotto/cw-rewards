use std::marker::PhantomData;

use crate::{
  error::ContractError,
  models::{GrantQueryResult, RewardKey},
  msg::GrantsResponse,
  state::{GRANTS, REWARDS},
};
use cosmwasm_std::{Addr, Deps, Order};
use cw_storage_plus::Bound;

pub fn get_grants(
  deps: Deps,
  wallet: &Addr,
  some_limit: Option<u8>,
  some_cursor: Option<RewardKey>,
) -> Result<GrantsResponse, ContractError> {
  let limit = some_limit.unwrap_or(20).clamp(1, 20) as usize;
  let start = (|| -> Result<_, ContractError> {
    if let Some(reward_key) = some_cursor {
      Ok(Some(Bound::Inclusive((reward_key, PhantomData))))
    } else {
      Err(ContractError::NotAuthorized {})
    }
  })()?;

  let mut results: Vec<GrantQueryResult> = vec![];

  for key_result in GRANTS
    .prefix(wallet.clone())
    .keys(deps.storage, start, None, Order::Descending)
    .take(limit)
  {
    if let Ok(reward_key) = key_result {
      let (sender, key) = reward_key.clone();
      if let Some(meta) = REWARDS.may_load(deps.storage, reward_key.clone())? {
        results.push(GrantQueryResult {
          claims_processed: meta.claims_processed,
          claims_remaining: meta.claims_remaining,
          message: meta.message,
          sender,
          key,
        });
      } else {
        return Err(ContractError::RewardNotFound {});
      }
    }
  }

  let cursor = if !results.is_empty() {
    if let Some(last_reward) = results.last() {
      Some((last_reward.sender.clone(), last_reward.key.clone()))
    } else {
      None
    }
  } else {
    None
  };

  Ok(GrantsResponse {
    rewards: results,
    cursor,
  })
}

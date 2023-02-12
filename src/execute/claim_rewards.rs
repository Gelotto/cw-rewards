use std::collections::HashMap;

use crate::{
  error::ContractError,
  models::{ClaimTotals, GrantKey, RewardKey},
  msg::ClaimableExecuteMsgInterface,
  state::{CLAIM_TOTALS, GRANTS, REWARDS},
};
use cosmwasm_std::{attr, to_binary, Addr, DepsMut, Env, MessageInfo, Response, WasmMsg};
use cw_lib::models::{Token, TokenAmount};

pub fn claim_rewards(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  reward_keys: Vec<RewardKey>,
) -> Result<Response, ContractError> {
  let claimant = &info.sender;
  let mut contract_addr_2_token_amounts: HashMap<Addr, HashMap<(bool, String), TokenAmount>> =
    HashMap::new();

  // prepare data required for building subsequent claim messages
  for reward_key in reward_keys.iter() {
    let (contract_addr, _) = reward_key;
    let grant_key: GrantKey = (claimant.clone(), reward_key.clone());

    // abort if grant expired or does not exist
    if let Some(grant) = GRANTS.may_load(deps.storage, grant_key)? {
      if grant.is_expired(&env.block) {
        return Err(ContractError::GrantExpired {});
      }
    } else {
      return Err(ContractError::GrantNotFound {});
    }

    // ensure a TokenAmount vec exists for the given contract addr
    if !contract_addr_2_token_amounts.contains_key(contract_addr) {
      contract_addr_2_token_amounts.insert(contract_addr.clone(), HashMap::new());
    }
    // we will be updating the contants of this amounts hashmap
    let amounts = contract_addr_2_token_amounts
      .get_mut(contract_addr)
      .unwrap();

    // update total amounts map with this reward
    if let Some(mut reward) = REWARDS.may_load(deps.storage, reward_key.clone())? {
      // skip paused rewards
      if reward.is_paused {
        continue;
      }
      // update claims_remaining counter
      if let Some(mut claims_remaining) = reward.claims_remaining {
        if claims_remaining == 0 {
          // we should never reach this, since a reward metadata should have been
          // removed by the claim that made claims_remaining 0 in the first place.
          return Err(ContractError::EmptyRewards {});
        }
        claims_remaining -= 1;
        reward.claims_remaining = Some(claims_remaining);
      }
      // always
      reward.claims_processed += 1;

      // increment token totals from this reward's contract

      for reward_ta in reward.tokens {
        let key = match &reward_ta.token {
          Token::Native { denom } => (false, denom.clone()),
          Token::Cw20 { address } => (false, address.to_string()),
        };
        if let Some(total_ta) = amounts.get(&key) {
          let mut new_total_ta = total_ta.clone();
          new_total_ta.amount += reward_ta.amount;
          amounts.insert(key, new_total_ta);
        } else {
          amounts.insert(key, reward_ta.clone());
        }
      }
    }
  }

  // build claim messages for all contracts with pending claims
  let mut claim_msgs: Vec<WasmMsg> = Vec::with_capacity(contract_addr_2_token_amounts.len());
  for (contract_addr, amounts_hashmap) in contract_addr_2_token_amounts.iter() {
    // build claim WASM msg
    claim_msgs.push(WasmMsg::Execute {
      contract_addr: contract_addr.into(),
      msg: to_binary(&ClaimableExecuteMsgInterface::Claim {
        tokens: Some(amounts_hashmap.values().map(|x| x.clone()).collect()),
      })?,
      funds: vec![],
    });

    // increment global historical claim totals for this claimant
    for ta in amounts_hashmap.values() {
      let token_key = match &ta.token {
        Token::Native { denom } => denom.clone(),
        Token::Cw20 { address } => address.to_string(),
      };
      CLAIM_TOTALS.update(
        deps.storage,
        (claimant.clone(), token_key),
        |some_totals| -> Result<ClaimTotals, ContractError> {
          if let Some(mut totals) = some_totals {
            totals.amount_claimed += ta.amount;
            Ok(totals)
          } else {
            // we should never reach this, since the ClaimTotals should have been
            // created upon creating the grant
            Err(ContractError::NotAuthorized {})
          }
        },
      )?;
    }
  }

  Ok(
    Response::new()
      .add_attributes(vec![attr("action", "claim")])
      .add_messages(claim_msgs),
  )
}

use crate::{
  error::ContractError,
  models::{ClaimTotals, Expiration, Grant, GrantKey},
  state::{CLAIM_TOTALS, GRANTS, REWARDS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response, Storage, Uint128};
use cw_lib::models::Token;

/// Grant acceess to reward keys to a specified wallet acount. This function
/// should only be executable by a contract authorized to have previously
/// registered the corresponding rewards.
pub fn grant_rewards(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  grantee: &Addr,
  keys: Vec<String>,
  expires_at: Option<Expiration>,
) -> Result<Response, ContractError> {
  for key in keys.iter() {
    grant_reward(deps.storage, &env, &info, grantee, key, &expires_at)?;
  }
  Ok(Response::new().add_attributes(vec![attr("action", "grant")]))
}

fn grant_reward(
  storage: &mut dyn Storage,
  _env: &Env,
  info: &MessageInfo,
  grantee: &Addr,
  key: &String,
  expires_at: &Option<Expiration>,
) -> Result<(), ContractError> {
  let reward_key = (info.sender.clone(), key.clone());
  if let Some(reward) = REWARDS.may_load(storage, reward_key)? {
    let grant_key: GrantKey = (grantee.clone(), (info.sender.clone(), key.clone()));

    for ta in reward.tokens.iter() {
      let token_key = match &ta.token {
        Token::Native { denom } => denom.clone(),
        Token::Cw20 { address } => address.to_string(),
      };
      CLAIM_TOTALS.update(
        storage,
        (grantee.clone(), token_key),
        |some_totals| -> Result<ClaimTotals, ContractError> {
          if let Some(mut totals) = some_totals {
            totals.amount_granted += ta.amount;
            Ok(totals)
          } else {
            Ok(ClaimTotals {
              token: ta.token.clone(),
              amount_claimed: Uint128::zero(),
              amount_granted: ta.amount,
            })
          }
        },
      )?;
    }

    GRANTS.save(
      storage,
      grant_key,
      &Grant {
        expires_at: expires_at.clone(),
      },
    )?;
    Ok(())
  } else {
    // reward not found
    return Err(ContractError::RewardNotFound {});
  }
}

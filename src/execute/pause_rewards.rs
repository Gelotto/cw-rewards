use crate::{error::ContractError, state::REWARDS};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn pause_rewards(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  keys: Vec<String>,
) -> Result<Response, ContractError> {
  for key in keys.iter() {
    let reward_key = (info.sender.clone(), key.clone());
    if let Some(mut reward) = REWARDS.may_load(deps.storage, reward_key.clone())? {
      reward.is_paused = true;
      REWARDS.save(deps.storage, reward_key, &reward)?;
    }
  }
  Ok(Response::new().add_attributes(vec![attr("action", "pause")]))
}

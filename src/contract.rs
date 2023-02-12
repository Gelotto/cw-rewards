#[cfg(not(feature = "library"))]
use crate::error::ContractError;
use crate::execute;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-rewards";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::Register { rewards } => execute::register_rewards(deps, env, info, rewards),
    ExecuteMsg::Grant {
      grantee,
      keys,
      expiration,
    } => execute::grant_rewards(deps, env, info, &grantee, keys, expiration),
    ExecuteMsg::Revoke { grantee, keys } => {
      execute::revoke_rewards(deps, env, info, &grantee, keys)
    },
    ExecuteMsg::Pause { keys } => execute::pause_rewards(deps, env, info, keys),
    ExecuteMsg::Claim { grants } => execute::claim_rewards(deps, env, info, grants),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> Result<Binary, ContractError> {
  let result = match msg {
    QueryMsg::Select { fields } => to_binary(&query::select(deps, fields)?),
    QueryMsg::Rewards {
      contract,
      limit,
      cursor,
    } => to_binary(&query::get_rewards(deps, &contract, limit, cursor)?),
    QueryMsg::Grants {
      wallet,
      limit,
      cursor,
    } => to_binary(&query::get_grants(deps, &wallet, limit, cursor)?),
  }?;
  Ok(result)
}

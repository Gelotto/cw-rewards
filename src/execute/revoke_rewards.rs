use crate::{error::ContractError, models::GrantKey, state::GRANTS};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

/// Grant acceess to reward keys to a specified wallet acount. This function
/// should only be executable by a contract authorized to have previously
/// registered the corresponding rewards.
pub fn revoke_rewards(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  grantee: &Addr,
  keys: Vec<String>,
) -> Result<Response, ContractError> {
  for key in keys.iter() {
    let grant_key: GrantKey = (grantee.clone(), (info.sender.clone(), key.clone()));
    GRANTS.remove(deps.storage, grant_key);
  }
  Ok(Response::new().add_attributes(vec![attr("action", "revoke")]))
}

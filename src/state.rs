use crate::models::{ClaimTotals, Grant, GrantKey, RewardMetadata};
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::RewardKey};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, StdResult, Storage};
use cw_storage_plus::{Item, Map};

pub const CREATED_BY: Item<Addr> = Item::new("created_by");
pub const REWARDS: Map<RewardKey, RewardMetadata> = Map::new("rewards");
pub const GRANTS: Map<GrantKey, Grant> = Map::new("grants");
pub const CLAIM_TOTALS: Map<(Addr, String), ClaimTotals> = Map::new("claim_totals");

pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  _msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  CREATED_BY.save(deps.storage, &info.sender)?;
  Ok(())
}

pub fn is_owner(
  storage: &dyn Storage,
  addr: &Addr,
) -> StdResult<bool> {
  return Ok(CREATED_BY.load(storage)? == *addr);
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_lib::models::TokenAmount;

use crate::models::{Expiration, GrantQueryResult, RewardInitArgs, RewardKey, RewardQueryResult};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
  Grant {
    grantee: Addr,
    keys: Vec<String>,
    expiration: Option<Expiration>,
  },
  Revoke {
    grantee: Addr,
    keys: Vec<String>,
  },
  Register {
    rewards: Vec<RewardInitArgs>,
  },
  Claim {
    grants: Vec<RewardKey>,
  },
  Pause {
    keys: Vec<String>,
  },
}

#[cw_serde]
pub enum QueryMsg {
  Select {
    fields: Option<Vec<String>>,
  },
  Rewards {
    contract: Addr,
    limit: Option<u8>,
    cursor: Option<String>,
  },
  Grants {
    wallet: Addr,
    limit: Option<u8>,
    cursor: Option<RewardKey>,
  },
}

#[cw_serde]
pub struct SelectResponse {
  pub created_by: Option<Addr>,
}

#[cw_serde]
pub struct GrantsResponse {
  pub rewards: Vec<GrantQueryResult>,
  pub cursor: Option<RewardKey>,
}

#[cw_serde]
pub struct RewardsResponse {
  pub rewards: Vec<RewardQueryResult>,
  pub cursor: Option<String>,
}

#[cw_serde]
pub enum ClaimableExecuteMsgInterface {
  Claim { tokens: Option<Vec<TokenAmount>> },
}

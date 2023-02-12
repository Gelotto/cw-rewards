use crate::{
  error::ContractError,
  models::{Expiration, RewardInitArgs, RewardMetadata},
  state::REWARDS,
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Storage};

// TODO: use ACL to authorize add_rewards and other functions that must be
// executed by only authorized contracts. This ACL should be updated to allow
// each contract upon instantiation by an authorized Collection contract.

pub fn register_rewards(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  rewards: Vec<RewardInitArgs>,
) -> Result<Response, ContractError> {
  for args in rewards.iter() {
    register_reward(deps.storage, &env, &info, args)?;
  }
  Ok(Response::new().add_attributes(vec![attr("action", "register")]))
}

fn register_reward(
  storage: &mut dyn Storage,
  env: &Env,
  info: &MessageInfo,
  args: &RewardInitArgs,
) -> Result<(), ContractError> {
  // validate reward expiry
  if let Some(expires_at) = &args.expires_at {
    match expires_at {
      Expiration::Time(time) => {
        if *time <= env.block.time {
          return Err(ContractError::InvalidExpiration {});
        }
      },
      Expiration::Height(height) => {
        if *height <= env.block.height {
          return Err(ContractError::InvalidExpiration {});
        }
      },
    }
  }

  // disallow empty reward amounts
  if args.tokens.len() == 0 || args.tokens.iter().any(|x| x.amount.is_zero()) {
    return Err(ContractError::InvalidRewardAmount {});
  }

  // persist reward metadata
  REWARDS.update(
    storage,
    (info.sender.clone(), args.key.clone()),
    |existing_meta| -> Result<RewardMetadata, ContractError> {
      if existing_meta.is_some() {
        return Err(ContractError::AlreadyExists {});
      }
      Ok(RewardMetadata {
        claims_processed: 0,
        is_paused: false,
        message: args.message.clone(),
        claims_remaining: args.count,
        tokens: args.tokens.clone(),
      })
    },
  )?;
  Ok(())
}

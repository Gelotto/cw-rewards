use cosmwasm_std::StdError;
use thiserror::Error;

// TODO: Add messages/data to errors

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("NotAuthorized")]
  NotAuthorized {},

  #[error("AlreadyExists")]
  AlreadyExists {},

  #[error("CursorOutOfBounds")]
  CursorOutOfBounds {},

  #[error("GrantExpired")]
  GrantExpired {},

  #[error("GrantNotFound")]
  GrantNotFound {},

  #[error("RewardNotFound")]
  RewardNotFound {},

  #[error("EmptyRewards")]
  EmptyRewards {},

  #[error("InvalidRewardAmout")]
  InvalidRewardAmount {},

  #[error("ValidationError")]
  ValidationError {},

  #[error("InvalidExpiration")]
  InvalidExpiration {},
}

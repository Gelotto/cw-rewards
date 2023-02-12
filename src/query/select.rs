use crate::{msg::SelectResponse, state::CREATED_BY};
use cosmwasm_std::{Deps, StdResult};
use cw_repository::client::Repository;

pub fn select(
  deps: Deps,
  fields: Option<Vec<String>>,
) -> StdResult<SelectResponse> {
  let loader = Repository::loader(deps.storage, &fields);
  Ok(SelectResponse {
    created_by: loader.get("created_by", &CREATED_BY)?,
  })
}

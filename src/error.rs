use cosmwasm_std::StdError;
use thiserror::Error;

use crate::state::mapIndexType;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Member not found for index {0}")]
    MemberNotFound(mapIndexType),

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

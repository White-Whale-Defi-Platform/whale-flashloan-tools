use cosmwasm_std::StdError;
use thiserror::Error;

use cw_controllers::AdminError;


#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Sender of callback is not the registered Vault.")]
    NotValidCallback {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
// Handler semver errors related to upgradability
impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
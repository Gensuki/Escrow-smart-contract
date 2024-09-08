use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized: Only the admin can perform this action")]
    Unauthorized {},

    #[error("Invalid input: {msg}")]
    InvalidInput { msg: String },

    #[error("NFT transfer failed")]
    NftTransferFailed {},

    #[error("Insufficient funds")]
    InsufficientFunds {},
}

use cosmwasm_std::{Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    TransferNative { to_address: String, amount: Coin },
    TransferCw20 {
        contract_addr: String, // Address of the CW20 contract
        to_address: String,
        amount: Uint128,
    },
    ApproveNftTransfer {
        nft_contract_addr: String, // Address of the CW721 contract
        token_id: String,
    },
    TransferNft { nft_contract_addr: String ,to_address: String, token_id: String },
    TransferNfts {
        nft_contract_addr: String, // Address of the CW721 contract
        to_address: String,
        token_ids: Vec<String>,
    },
    BulkTransferNative { to_addresses: Vec<String>, amounts: Vec<Coin> },
    BulkTransferCw20 { cw20_contract_addr: String, to_addresses: Vec<String>, amounts: Vec<Uint128> },
    SetOperator { operator: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetAdmin {},
    GetTotalEscrowNfts { nft_contract_addrs: Vec<String>, limit: Option<u32> },
    GetEscrowNfts { nft_contract_addr: String, limit: u32 },
    GetCw20Tokens { cw20_contract_addr: String },
    GetNativeTokens {},
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryResponse {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetTotalEscrowNftsResponse {
    pub token_ids: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetEscrowNftsResponse {
    pub nft_contract_addr: String,  // NFT contract address
    pub nfts: Vec<String>,          // List of token IDs held by the contract
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetCw20TokensResponse {
    pub cw20_contract_addr: String,  // CW20 contract address
    pub balance: Uint128,            // Total balance of CW20 tokens
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTokensResponse {
    pub tokens: Vec<Coin>,  // List of native token balances (denom, amount)
}
use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, BankMsg, WasmMsg, to_json_binary, Coin, Binary, Uint128};
use cw_storage_plus::Item;
use cw721::Cw721ExecuteMsg;
use cw721::{Cw721QueryMsg, TokensResponse};
use cw20::Cw20ExecuteMsg;
use cw20::{Cw20QueryMsg, BalanceResponse};
use crate::msg::{ExecuteMsg, GetTotalEscrowNftsResponse, InstantiateMsg, QueryMsg, QueryResponse};

// Admin address is stored in contract state
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const OPERATOR: Item<Option<Addr>> = Item::new("operator");

// Instantiate the contract with the admin address
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let admin = info.sender.clone();
    ADMIN.save(deps.storage, &admin)?;
    OPERATOR.save(deps.storage, &None)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", admin))
}

// Check if sender is admin
pub fn only_admin_or_operator(deps: Deps, info: &MessageInfo) -> StdResult<()> {
    let admin = ADMIN.load(deps.storage)?;
    let operator = OPERATOR.load(deps.storage)?;

    // Check if the sender is either the admin or a valid operator
    if info.sender != admin && (operator.is_none() || operator.as_ref() != Some(&info.sender)) {
        return Err(StdError::generic_err("Unauthorized: Only admin or operator can perform this action"));
    }
    Ok(())
}



pub fn set_operator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    operator: Option<String>,
) -> StdResult<Response> {
    let admin = ADMIN.load(deps.storage)?;

    // Only admin can set or remove operator
    if info.sender != admin {
        return Err(StdError::generic_err("Unauthorized: Only admin can set or remove operator"));
    }

    // Validate and save the new operator address, if provided
    let new_operator = match operator {
        Some(addr) => Some(deps.api.addr_validate(&addr)?),
        None => None,
    };

    OPERATOR.save(deps.storage, &new_operator)?;

    // Create a response indicating the action taken and the current operator status
    Ok(Response::new()
        .add_attribute("action", "set_operator")
        .add_attribute("operator", new_operator.map_or("none".to_string(), |o| o.to_string())))
}


// Bulk native token transfer
pub fn bulk_transfer_native(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to_addresses: Vec<String>,
    amounts: Vec<Coin>,
) -> StdResult<Response> {
    only_admin_or_operator(deps.as_ref(), &info)?;

    if to_addresses.len() != amounts.len() {
        return Err(StdError::generic_err("Mismatched addresses and amounts"));
    }

    let mut messages: Vec<BankMsg> = Vec::new();

    for (i, to_address) in to_addresses.iter().enumerate() {
        let transfer_msg = BankMsg::Send {
            to_address: to_address.clone(),
            amount: vec![amounts[i].clone()],
        };
        messages.push(transfer_msg);
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "bulk_transfer_native"))
}


// Bulk CW20 token transfer
pub fn bulk_transfer_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_contract_addr: String,
    to_addresses: Vec<String>,
    amounts: Vec<Uint128>,
) -> StdResult<Response> {
    only_admin_or_operator(deps.as_ref(), &info)?;

    if to_addresses.len() != amounts.len() {
        return Err(StdError::generic_err("Mismatched addresses and amounts"));
    }

    let mut messages: Vec<WasmMsg> = Vec::new();

    for (i, to_address) in to_addresses.iter().enumerate() {
        let cw20_transfer_msg = WasmMsg::Execute {
            contract_addr: deps.api.addr_validate(&cw20_contract_addr)?.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: to_address.clone(),
                amount: amounts[i],
            })?,
            funds: vec![],
        };
        messages.push(cw20_transfer_msg);
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "bulk_transfer_cw20"))
}


// Transfer native tokens (SEI) to a specified address
pub fn transfer_native(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to_address: String,
    amount: Coin,
) -> StdResult<Response> {
    only_admin_or_operator(deps.as_ref(), &info)?;

    // Logic to transfer native coin
    let transfer_msg = BankMsg::Send {
        to_address: to_address.clone(),
        amount: vec![amount.clone()],
    };

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "transfer_native")
        .add_attribute("to_address", to_address)
        .add_attribute("amount", amount.to_string()))
}


//Tranfer cw20 tokens
pub fn transfer_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract_addr: String, // Address of the CW20 token contract
    to_address: String,
    amount: Uint128, // Amount of tokens to transfer
) -> StdResult<Response> {
    // Ensure only admin can perform this action
    only_admin_or_operator(deps.as_ref(), &info)?;

    // Construct the CW20 transfer message
    let cw20_transfer_msg = WasmMsg::Execute {
        contract_addr: deps.api.addr_validate(&contract_addr)?.to_string(),
        msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
            recipient: to_address.clone(),
            amount,
        })?,
        funds: vec![],
    };

    // Return the response with the transfer message
    Ok(Response::new()
        .add_message(cw20_transfer_msg)
        .add_attribute("action", "transfer_cw20")
        .add_attribute("to_address", to_address)
        .add_attribute("amount", amount.to_string()))
}

pub fn approve_nft_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    nft_contract_addr: String,
    token_id: String,
) -> StdResult<Response> {
    only_admin_or_operator(deps.as_ref(), &info)?;

    // Construct the CW721 approve message
    let approve_msg = WasmMsg::Execute {
        contract_addr: deps.api.addr_validate(&nft_contract_addr)?.to_string(),
        msg: to_json_binary(&Cw721ExecuteMsg::Approve {
            spender: _env.contract.address.to_string(),
            token_id: token_id.clone(),
            expires: None,
        })?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(approve_msg)
        .add_attribute("action", "approve_nft_transfer")
        .add_attribute("token_id", token_id))
}


// Transfer an NFT (CW721) to a specified address
pub fn transfer_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    nft_contract_addr: String,
    to_address: String,
    token_id: String,
) -> StdResult<Response> {
    only_admin_or_operator(deps.as_ref(), &info)?;

    // Transfer an NFT using the CW721 contract
    let nft_transfer_msg = WasmMsg::Execute {
        contract_addr: deps.api.addr_validate(&nft_contract_addr)?.to_string(), // Change this to your CW721 contract address
        msg: to_json_binary(&Cw721ExecuteMsg::TransferNft {
            recipient: to_address.clone(),
            token_id: token_id.clone(),
        })?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(nft_transfer_msg)
        .add_attribute("action", "transfer_nft")
        .add_attribute("to_address", to_address)
        .add_attribute("token_id", token_id))
}

pub fn transfer_nfts(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    nft_contract_addr: String,
    to_address: String,
    token_ids: Vec<String>,
) -> StdResult<Response> {
    only_admin_or_operator(deps.as_ref(), &info)?;

    let mut messages: Vec<WasmMsg> = Vec::new();
    
    // Iterate over token IDs and create transfer messages
    for token_id in &token_ids {
        let approve_msg = WasmMsg::Execute {
            contract_addr: deps.api.addr_validate(&nft_contract_addr)?.to_string(),
            msg: to_json_binary(&Cw721ExecuteMsg::Approve {
                spender: _env.contract.address.to_string(),
                token_id: token_id.clone(),
                expires: None,
            })?,
            funds: vec![],
        };

        let transfer_msg = WasmMsg::Execute {
            contract_addr: deps.api.addr_validate(&nft_contract_addr)?.to_string(),
            msg: to_json_binary(&Cw721ExecuteMsg::TransferNft {
                recipient: to_address.clone(),
                token_id: token_id.clone(),
            })?,
            funds: vec![],
        };

        messages.push(approve_msg);
        messages.push(transfer_msg);
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "transfer_nfts")
        .add_attribute("to_address", to_address)
        .add_attribute("token_ids", token_ids.join(", ")))
}

// Implement the execute function
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::TransferNative { to_address, amount } => {
            transfer_native(deps, _env, info, to_address, amount)
        }
        ExecuteMsg::TransferCw20 { contract_addr, to_address, amount } => {
            transfer_cw20(deps, _env, info, contract_addr, to_address, amount)
        }
        ExecuteMsg::ApproveNftTransfer { nft_contract_addr, token_id } => {
            approve_nft_transfer(deps, _env, info, nft_contract_addr, token_id)
        }
        ExecuteMsg::TransferNft { nft_contract_addr, to_address, token_id } => {
            transfer_nft(deps, _env, info, nft_contract_addr, to_address, token_id)
        }
        ExecuteMsg::TransferNfts { nft_contract_addr, to_address, token_ids } => {
            transfer_nfts(deps, _env, info, nft_contract_addr, to_address, token_ids)
        }
        ExecuteMsg::BulkTransferNative { to_addresses, amounts } => {
            bulk_transfer_native(deps, _env, info, to_addresses, amounts)
        }
        ExecuteMsg::BulkTransferCw20 { cw20_contract_addr, to_addresses, amounts } => {
            bulk_transfer_cw20(deps, _env, info, cw20_contract_addr, to_addresses, amounts)
        }
        ExecuteMsg::SetOperator { operator } => {
            set_operator(deps, _env, info, operator)
        }
    }
}

pub fn get_total_escrow_nfts(
    deps: Deps,
    _env: Env,
    nft_contract_addrs: Vec<String>,
    limit: Option<u32>
) -> StdResult<GetTotalEscrowNftsResponse> {
    let mut all_tokens = Vec::new();

    for addr in nft_contract_addrs {
        let nft_contract = deps.api.addr_validate(&addr)?;
        let query_msg = Cw721QueryMsg::Tokens {
            owner: _env.contract.address.to_string(),
            start_after: None,
            limit: limit.map(|l| l as u32),
        };

        // Query each contract and collect the token IDs
        let tokens: TokensResponse = deps.querier.query_wasm_smart(nft_contract, &query_msg)?;
        all_tokens.extend(tokens.tokens);
    }

    Ok(GetTotalEscrowNftsResponse { token_ids: all_tokens })
}


pub fn get_escrow_nfts(
    deps: Deps,
    env: Env,  // Remove the underscore since we're using it
    nft_contract_addr: String,
    limit: u32,
) -> StdResult<Binary> {
    let nft_contract = deps.api.addr_validate(&nft_contract_addr)?;

    // Create the query message with the contract address as the owner
    let query_msg = Cw721QueryMsg::Tokens {
        owner: env.contract.address.to_string(),
        start_after: None,
        limit: Some(limit),
    };

    // Query the CW721 contract for tokens
    let tokens: TokensResponse = deps.querier.query_wasm_smart(nft_contract, &query_msg)?;

    // Return the list of token IDs as binary
    to_json_binary(&tokens)
}

pub fn get_cw20_tokens(
    deps: Deps,
    env: Env, // Remove the underscore since we need it
    cw20_contract_addr: String,
) -> StdResult<Binary> {
    let cw20_contract = deps.api.addr_validate(&cw20_contract_addr)?;
    
    // Create the query message
    let query_msg = Cw20QueryMsg::Balance {
        address: env.contract.address.to_string(),
    };

    // Query the CW20 contract for balance
    let balance: BalanceResponse = deps.querier.query_wasm_smart(cw20_contract, &query_msg)?;

    // Return the balance as binary
    to_json_binary(&balance)
}


pub fn get_native_tokens(
    deps: Deps,
    _env: Env,
) -> StdResult<Binary> {
    // Query the contract balance of SEI tokens
    let balance = deps.querier.query_balance(&_env.contract.address, "usei")?;
    
    to_json_binary(&balance)
}


// Implement the query function
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => {
            let admin = ADMIN.load(deps.storage)?;
            to_json_binary(&QueryResponse { admin: admin.to_string() })
        }
      
        QueryMsg::GetTotalEscrowNfts { nft_contract_addrs, limit } => {
            let response = get_total_escrow_nfts(deps, _env, nft_contract_addrs, limit)?;
            to_json_binary(&response)
        }
        QueryMsg::GetEscrowNfts { nft_contract_addr, limit } => {
            get_escrow_nfts(deps, _env, nft_contract_addr, limit)
        }
        QueryMsg::GetCw20Tokens { cw20_contract_addr } => {
            get_cw20_tokens(deps, _env, cw20_contract_addr)
        }
        QueryMsg::GetNativeTokens {} => {
            get_native_tokens(deps, _env)
        }
    }
}

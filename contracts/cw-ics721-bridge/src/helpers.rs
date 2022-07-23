use crate::state::CLASS_ID_TO_NFT_CONTRACT;
use crate::ContractError;
use cosmwasm_std::{to_binary, Addr, Deps, DepsMut, Empty, StdResult, SubMsg, WasmMsg};
use cw721::{NftInfoResponse, OwnerOfResponse};

pub const MINT_SUB_MSG_REPLY_ID: u64 = 0;
pub const TRANSFER_SUB_MSG_REPLY_ID: u64 = 1;
pub const BURN_SUB_MSG_REPLY_ID: u64 = 2;
pub const INSTANTIATE_AND_MINT_CW721_REPLY_ID: u64 = 3;
pub const INSTANTIATE_CW721_REPLY_ID: u64 = 4;
pub const INSTANTIATE_ESCROW_REPLY_ID: u64 = 5;
pub const FAILURE_RESPONSE_FAILURE_REPLY_ID: u64 = 6;
pub const BATCH_TRANSFER_FROM_CHANNEL_REPLY_ID: u64 = 7;
pub const BURN_ESCROW_TOKENS_REPLY_ID: u64 = 8;

pub fn mint(
    deps: DepsMut,
    class_id: String,
    token_id: String,
    token_uri: String,
    receiver: String,
) -> Result<SubMsg, ContractError> {
    if !CLASS_ID_TO_NFT_CONTRACT.has(deps.storage, class_id.clone()) {
        return Err(ContractError::UnrecognisedClassId {});
    }

    let class_uri = CLASS_ID_TO_NFT_CONTRACT.load(deps.storage, class_id)?;
    let mint_msg = cw721_base::ExecuteMsg::<Empty>::Mint(cw721_base::MintMsg::<Empty> {
        token_id,
        owner: receiver,
        token_uri: Some(token_uri),
        extension: Empty {},
    });
    let msg = WasmMsg::Execute {
        contract_addr: class_uri.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    };
    let msg = SubMsg::reply_always(msg, MINT_SUB_MSG_REPLY_ID);

    Ok(msg)
}

pub fn transfer(
    deps: Deps,
    class_id: String,
    token_id: String,
    receiver: String,
) -> Result<SubMsg, ContractError> {
    if !CLASS_ID_TO_NFT_CONTRACT.has(deps.storage, class_id.clone()) {
        return Err(ContractError::UnrecognisedClassId {});
    }
    // Validate receiver
    deps.api.addr_validate(&receiver)?;

    // No need to perform other checks as we can piggyback on cw721-base erroring for us

    let class_uri = CLASS_ID_TO_NFT_CONTRACT.load(deps.storage, class_id)?;
    let transfer_msg = cw721_base::ExecuteMsg::<Empty>::TransferNft {
        recipient: receiver,
        token_id,
    };
    let msg = WasmMsg::Execute {
        contract_addr: class_uri.to_string(),
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    };

    let msg = SubMsg::reply_always(msg, TRANSFER_SUB_MSG_REPLY_ID);
    Ok(msg)
}

pub fn burn(deps: Deps, class_id: String, token_id: String) -> Result<SubMsg, ContractError> {
    if !CLASS_ID_TO_NFT_CONTRACT.has(deps.storage, class_id.clone()) {
        return Err(ContractError::UnrecognisedClassId {});
    }

    let class_uri = CLASS_ID_TO_NFT_CONTRACT.load(deps.storage, class_id)?;
    let burn_msg = cw721_base::ExecuteMsg::<Empty>::Burn { token_id };
    let msg = WasmMsg::Execute {
        contract_addr: class_uri.to_string(),
        msg: to_binary(&burn_msg)?,
        funds: vec![],
    };

    let msg = SubMsg::reply_always(msg, BURN_SUB_MSG_REPLY_ID);
    Ok(msg)
}

pub fn get_owner(deps: Deps, class_id: String, token_id: String) -> StdResult<OwnerOfResponse> {
    let class_uri = CLASS_ID_TO_NFT_CONTRACT.load(deps.storage, class_id)?;
    let resp: OwnerOfResponse = deps.querier.query_wasm_smart(
        class_uri,
        &cw721_base::QueryMsg::OwnerOf {
            token_id,
            include_expired: None,
        },
    )?;
    Ok(resp)
}

pub fn get_nft(
    deps: Deps,
    class_id: String,
    token_id: String,
) -> StdResult<NftInfoResponse<Empty>> {
    let class_uri = CLASS_ID_TO_NFT_CONTRACT.load(deps.storage, class_id)?;
    let resp: NftInfoResponse<Empty> = deps
        .querier
        .query_wasm_smart(class_uri, &cw721_base::QueryMsg::NftInfo { token_id })?;
    Ok(resp)
}

pub fn has_class(deps: Deps, class_id: String) -> StdResult<bool> {
    Ok(CLASS_ID_TO_NFT_CONTRACT.has(deps.storage, class_id))
}

pub fn get_class(deps: Deps, class_id: String) -> StdResult<Addr> {
    CLASS_ID_TO_NFT_CONTRACT.load(deps.storage, class_id)
}
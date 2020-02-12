use named_type::NamedType;
use named_type_derive::NamedType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm::types::HumanAddr;

use crate::state::Amount;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct InitialBalance {
    pub address: HumanAddr,
    pub amount: Amount,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<InitialBalance>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum HandleMsg {
    Approve {
        spender: HumanAddr,
        amount: Amount,
    },
    Transfer {
        recipient: HumanAddr,
        amount: Amount,
    },
    TransferFrom {
        owner: HumanAddr,
        recipient: HumanAddr,
        amount: Amount,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum QueryMsg {
    Balance {
        address: HumanAddr,
    },
    Allowance {
        owner: HumanAddr,
        spender: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, NamedType)]
pub struct BalanceResponse {
    pub balance: Amount,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, NamedType)]
pub struct AllowanceResponse {
    pub allowance: Amount,
}
use std::{collections::HashMap, fmt};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp, Coin, Response};
use cw_storage_plus::{Map, Item};

use crate::ContractError;

type TYPE_TODO = bool;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub liveStatus: LiveStatus,
    pub group_type: GroupType,
    // group_members: TYPE_TODO,
    // group_rules: TYPE_TODO,
    pub expiry: Expiry,
    pub recovery: RecoveryInfo,
    pub credential: Credential,
    pub version: VersionInfo
}
pub type mapIndexType = u8; // Note: can't use usize due to weird floating point issue
pub const STATE: Item<State> = Item::new("state");
pub const MEMBERS: Map<mapIndexType, Member> = Map::new("members");
pub const RULES: Map<mapIndexType, Rule> = Map::new("rules");

// pub const MEMBERS: Item<Vec<Member>> = Item::new("members");
// pub const RULES: Item<Vec<Rule>> = Item::new("rules");

//dependent types:

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Sender {
    Controller,
    Member(mapIndexType)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Payment {
    toAddr: Addr,
    amount: HashMap::<u8, Coin>, // Z! HMMMM... Vec<Coin>, but serialises...? HOW DO I SOLVE THIS?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum LiveStatus {   // may be other states in future
    Dormant,
    Alive,
    Paused,
    Defunct
}

impl Default for LiveStatus {
    fn default() -> Self {
        LiveStatus::Dormant
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum GroupType {
    Simple,
}

impl Default for GroupType {
    fn default() -> Self {
        GroupType::Simple
    }
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum RecoveryInfo {
    NOT_IMPLEMENTED_YET,
}

impl Default for RecoveryInfo {
    fn default() -> Self {
        RecoveryInfo::NOT_IMPLEMENTED_YET
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Expiry {
    Never,
    At(Timestamp),
}

impl Default for Expiry {
    fn default() -> Self {
        Expiry::Never
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum VersionInfo {
    V1,
}

impl Default for VersionInfo {
    fn default() -> Self {
        VersionInfo::V1
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Credential {
    CREDENTIAL_TO_BE_DEFINED,
}

impl Default for Credential {
    fn default() -> Self {
        Credential::CREDENTIAL_TO_BE_DEFINED
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Member {
    credential: Credential, 
}

impl Member {
    pub fn new(credential: Credential) -> Self {
        Self {
            credential
        }
    }

    pub fn set_credential(&mut self, new_credential: Credential) -> Result<Response, ContractError> {
        self.credential = new_credential;
        Ok(Response::new())
    }
    
    pub fn check_payment_allowed(&self, payment: Payment) -> Result<PaymentStatus, ContractError> {
        todo!()
    }
}

pub type DenomString = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Rule {
    Limit{ coinLimits: Vec<SpendingLimit>  },
    LimitLess,  // !
    Undefined,
}

impl Default for Rule {
    fn default() -> Self {
        Rule::Undefined
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rule_str = match self {
            NO_RULE_DEFINED => "NO RULE DEFINED",
        };
        write!(f, "{rule_str}")
    }
}

// OTHER TYPES
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum PaymentStatus {
    OK,
    NeedsConfirmation,
    Rejected,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]

pub struct SpendingLimit {
    denom: DenomString,
    amount: u128,
    period_in_days: u32,
}
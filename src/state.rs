use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Map, Item};

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
pub type mapIndex_u8 = u8;
pub const STATE: Item<State> = Item::new("state");
pub const MEMBERS: Map<mapIndex_u8, Member> = Map::new("members");
pub const RULES: Map<mapIndex_u8, Rule> = Map::new("rules");

// pub const MEMBERS: Item<Vec<Member>> = Item::new("members");
// pub const RULES: Item<Vec<Rule>> = Item::new("rules");

//dependent types:
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
    TODO_ENUM,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum RecoveryInfo {
    NO_RECOVERY_IMPLEMENTED,
}

impl Default for RecoveryInfo {
    fn default() -> Self {
        RecoveryInfo::NO_RECOVERY_IMPLEMENTED
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
    NO_CREDENTIAL_DEFINED,
}

impl Default for Credential {
    fn default() -> Self {
        Credential::NO_CREDENTIAL_DEFINED
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Member {
    TODO_ENUM,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Rule {
    NO_RULE_DEFINED,
}

impl Default for Rule {
    fn default() -> Self {
        Rule::NO_RULE_DEFINED
    }
}
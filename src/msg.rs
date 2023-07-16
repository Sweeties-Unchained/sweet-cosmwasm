use cosmwasm_schema::{cw_serde, QueryResponses};

use cw_storage_plus::{Map};
use cosmwasm_std::{Timestamp};

use crate::state::*;

#[cw_serde]
pub struct InstantiateMsg {
    // STATE:
    pub live: bool, //should get converted into a LiveStatus in handler
    pub group_type: GroupType,
    // expiry: Option<TimeStamp>,   // TO DO: allow instatiate with...
    pub recovery: RecoveryInfo,
    pub credential: Credential,
    pub version: VersionInfo,

    //MEMBERS, RULES
    pub members: Vec<Member>,
    pub rules: Vec<Rule>
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetLiveStatusResponse)]
    GetLiveStatus {},
    #[returns(GetGroupTypeResponse)]
    GetGroupType {},
    #[returns(GetMembersResponse)]
    GetMembers {},
    #[returns(GetRulesResponse)]
    GetRules {},
    #[returns(GetExpiryResponse)]
    GetExpiry {},
    #[returns(GetRecoveryInfoResponse)]
    GetRecoveryInfo {},
    #[returns(GetVersionInfoResponse)]
    GetVersionInfo {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetLiveStatusResponse {
    pub live_status: LiveStatus,
}

#[cw_serde]
pub struct GetGroupTypeResponse {
    pub group_type: GroupType,
}

#[cw_serde]
pub struct GetMembersResponse {
    pub members: Vec<(mapIndex_u8, Member)>,    // hmmm.... !
}

#[cw_serde]
pub struct GetRulesResponse {
    pub rules: Vec<(mapIndex_u8, Rule)>,
}

#[cw_serde]
pub struct GetExpiryResponse {
    pub expiry: Expiry,
}

#[cw_serde]
pub struct GetRecoveryInfoResponse {
    pub recover_info: RecoveryInfo,
}

#[cw_serde]
pub struct GetVersionInfoResponse {
    pub version_info: VersionInfo,
}

/* #[cw_serde]
pub struct GetGroupTypeResponse {
    pub group_type: GroupType,
} */
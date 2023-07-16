use std::collections::HashMap;

use cosmwasm_schema::{cw_serde, QueryResponses};

use cw_storage_plus::{Map};
use cosmwasm_std::{Timestamp, Coin};

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

// Execution
#[cw_serde]
pub enum ExecuteMsg {
    ControlMsg { control_msg: ControllerMsg, credential: Credential },  // messages from the "controller" (set up and edit app)
    MemberMsg { member_msg: MemberMsg, idx: mapIndexType, credential: Credential }, // messages from a user
}

#[cw_serde]
pub enum ControllerMsg {
    SetLiveStatus { live_status: LiveStatus },
    Update { 
        live_status: LiveStatus,
        group_type: GroupType,
        expiry: Expiry,
        recovery: RecoveryInfo,
        credential: Credential,
        version: VersionInfo,
    },
    AddMember { member: Member, credential: Credential },
    RemoveMember { m_idx: mapIndexType },
    AddRule { rule: Rule },
    RemoveRule { rule_idx: mapIndexType},
    ReplaceCredential {  new_credential: Credential },
    Test {},
}

#[cw_serde]
pub enum MemberMsg {
    Spend { payment: Payment },
    PayIn { coins: Vec<Coin> },   //TODO?
    ReplaceCredential { new_credential: Credential },
    // Propose { proposition: Proposition },  // not implemented!
    // Accept { proposition: Proposition },  // not implemented!
    // Sign { proposition: Proposition }, ,  // not implemented!
    // Reject { proposition: Proposition },   // not implemented!
    // Cancel { proposition: Proposition },   // not implemented!


    Test {}
}


//Queries

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

    //other
    #[returns(GetBalancesResponse)]
    GetBalances {},

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
    pub members: Vec<(mapIndexType, Member)>,    // hmmm.... !
}

#[cw_serde]
pub struct GetRulesResponse {
    pub rules: Vec<(mapIndexType, Rule)>,
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

#[cw_serde]
pub struct GetBalancesResponse {
    pub balances: HashMap<mapIndexType, Coin>, //hmm..?
}

/* #[cw_serde]
pub struct GetGroupTypeResponse {
    pub group_type: GroupType,
} */
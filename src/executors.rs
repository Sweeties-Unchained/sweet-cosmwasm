use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Empty, Coin};
use cw_multi_test::Contract;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetGroupTypeResponse, InstantiateMsg, QueryMsg, ControllerMsg, MemberMsg};
use crate::state::*;

/* 
example:
pub mod executors {
  use super::*;

pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}

pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("action", "reset"))
  }
}
*/


pub fn check_credentials(senderType: Sender, cred: Credential) -> Result<Response, ContractError> {
  match senderType {
    Sender::Controller => todo!(),
    Sender::Member(idx) => todo!()
  }
}

pub fn controller_msg_handler(deps: DepsMut, _env: Env, info: MessageInfo, control_msg: ControllerMsg) -> Result<Response, ContractError> {
  let res: Response = match control_msg {
    ControllerMsg::Test {} => Response::new().add_attribute("test controller msg", "ok!"),
    ControllerMsg::SetLiveStatus { live_status } => set_live_status(live_status, deps)?,
    ControllerMsg::AddMember { member , credential} => add_member(credential, deps)?,
    ControllerMsg::RemoveMember { m_idx } => remove_member(m_idx, deps)?,
    ControllerMsg::AddRule { rule } => add_rule(rule, deps)?,
    ControllerMsg::RemoveRule { rule_idx } => remove_rule(rule_idx, deps)?,
    // _ => todo!()
  };
  Ok(res)
}

pub fn member_msg_handler(deps: DepsMut, _env: Env, info: MessageInfo, m_idx: mapIndexType, member_msg: MemberMsg) -> Result<Response, ContractError> {
  let res: Response = match (m_idx, member_msg) {
    (_, MemberMsg::Test {}) => Response::new().add_attribute("test member msg", "ok!"),
    (m_idx, MemberMsg::ReplaceCredential { new_credential }) => replace_credential(m_idx, new_credential, deps)?,
    (m_idx, MemberMsg::Spend { payment }) => spend(m_idx, payment, deps)?,
    (m_idx, MemberMsg::PayIn { coins }) => pay_in(m_idx, coins, deps)?,

    // _ => todo!()
  };
  Ok(res)
}

// the actual handlers
pub fn set_live_status(new_live_status: LiveStatus, deps: DepsMut) -> Result<Response, ContractError> {
  STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
      state.liveStatus = new_live_status;
      Ok(state)
  })?;

  Ok(Response::new().add_attribute("action", "increment"))
}
/* pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
  ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
      let admins = admins
          .into_iter()
          .filter(|admin| *admin != info.sender)
          .collect();
      Ok(admins)
  })?;

  Ok(Response::new())
}*/

pub fn add_member(credential: Credential, deps: DepsMut) -> Result<Response, ContractError> {
  let m_idx = todo!() // where is new idx from?
  MEMBERS.save(deps.storage, m_idx, &Member::new(credential))?;
}

pub fn remove_member(m_idx: mapIndexType, deps: DepsMut) -> Result<Response, ContractError> {
  MEMBERS.remove(deps.storage, m_idx);
  let res = Response::new()
  .add_attribute("controller action", "remove")
  .add_attribute("m_idx", m_idx.to_string().as_str());
  Ok(res)
}

pub fn add_rule(rule: Rule, deps: DepsMut) -> Result<Response, ContractError> {
  let r_idx = todo!(); // new idx ?
  RULES.save(deps.storage, r_idx, &Rule::new(Rule::default()))?;
}

pub fn remove_rule(rule_idx: mapIndexType, deps: DepsMut) -> Result<Response, ContractError> {
  let rule = RULES.load(deps.storage, rule_idx)?;
  RULES.remove(deps.storage, rule_idx);
  let res: Response = Response::new()
    .add_attribute("controller action", "remove rule")
    .add_attribute("rule_idx", rule_idx.to_string())
    .add_attribute("rule", rule.to_string());

  Ok(res)
}
/* pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
  ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
      let admins = admins
          .into_iter()
          .filter(|admin| *admin != info.sender)
          .collect();
      Ok(admins)
  })?;

  Ok(Response::new())
} */

// MEMBERS

pub fn replace_credential(m_idx: mapIndexType, new_credential: Credential, deps: DepsMut) -> Result<Response, ContractError> {
  MEMBERS.update(deps.storage, m_idx, |mut opt_member| -> Result<_, ContractError> {
      if let Some(mut member) = opt_member {
        member.set_credential(new_credential)?;
        return Ok(member);
      }
      Err(ContractError::MemberNotFound(m_idx))
  })?;

  Ok(Response::new().add_attribute("member action", "replace credential"))
}

pub fn spend(m_idx: mapIndexType, payment: Payment, deps: DepsMut) -> Result<Response, ContractError> {
  let member = MEMBERS.load(deps.storage, m_idx)?;
  let payment_check = member.check_payment_allowed(payment)?;
  let res: Response = match payment_check {
    PaymentStatus::OK => {
       let res: Response = make_payment(payment)?;
       res.add_attribute("member payment", "ok") // TO DO!
    },
    PaymentStatus::NeedsConfirmation => {
      todo!()
    },
    PaymentStatus::Rejected => {
      todo!()
    }
  };

  Ok(res.add_attribute("member action", "spend"))
}

fn make_payment(payment: Payment) -> Result<Response, ContractError> {
  todo!()
}

pub fn pay_in(m_idx: mapIndexType, coins: Vec<Coin>, deps: DepsMut) -> Result<Response, ContractError> {
  todo!()
}
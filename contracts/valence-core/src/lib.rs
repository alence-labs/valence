#![no_std]
extern crate alloc;

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Vec};
use valence_types::{CapacitySummary, NodeConfig, NodeRecord};

const MINIMUM_STAKE: i128 = 1_000_000;
const SLA_QUORUM: u32 = 3;
const REWARD_AMOUNT: i128 = 250_000;
const PENALTY_AMOUNT: i128 = 125_000;

#[contracttype]
enum DataKey {
    NodeRecord(Address),
    RegisteredOperators,
    TotalCpu,
    TotalGpu,
    TotalStorage,
    VoteRecord(Address, Address),
    VoteCount(Address, bool),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyRegistered = 1,
    NotRegistered = 2,
    InsufficientStake = 3,
    Unauthorized = 4,
    QuorumNotReached = 5,
}

#[contract]
pub struct ValenceCore;

#[contractimpl]
impl ValenceCore {
    pub fn register_node(env: Env, operator: Address, config: NodeConfig, stake_amount: i128) -> Result<(), ContractError> {
        if stake_amount < MINIMUM_STAKE {
            return Err(ContractError::InsufficientStake);
        }

        if has_record(&env, &operator) {
            return Err(ContractError::AlreadyRegistered);
        }

        let record = NodeRecord {
            operator: operator.clone(),
            config: config.clone(),
            stake: stake_amount,
            active: true,
            reward_points: 0,
        };

        set_record(&env, &operator, &record);
        append_operator(&env, &operator);
        accelerate_capacity(&env, &config, 1);
        Ok(())
    }

    pub fn update_node(env: Env, operator: Address, config: NodeConfig) -> Result<(), ContractError> {
        let mut record = get_record(&env, &operator).ok_or(ContractError::NotRegistered)?;
        if !record.active {
            return Err(ContractError::Unauthorized);
        }

        let previous = record.config.clone();
        record.config = config.clone();
        set_record(&env, &operator, &record);
        adjust_capacity(&env, &previous, &config);
        Ok(())
    }

    pub fn submit_sla_vote(env: Env, reporter: Address, operator: Address, active: bool) -> Result<bool, ContractError> {
        let mut record = get_record(&env, &operator).ok_or(ContractError::NotRegistered)?;
        let vote_key = DataKey::VoteRecord(operator.clone(), reporter.clone());
        if env.storage().instance().has(&vote_key) {
            return Ok(false);
        }

        env.storage().instance().set(&vote_key, &active);
        let count_key = DataKey::VoteCount(operator.clone(), active);
        let current: u32 = env.storage().instance().get(&count_key).unwrap_or(0);
        let next = current + 1;
        env.storage().instance().set(&count_key, &next);

        if next >= SLA_QUORUM {
            if active {
                record.stake = record.stake + REWARD_AMOUNT;
                record.reward_points = record.reward_points + 1;
                record.active = true;
            } else {
                record.stake = (record.stake - PENALTY_AMOUNT).max(0);
                record.active = false;
            }
            set_record(&env, &operator, &record);
            return Ok(true);
        }

        Ok(false)
    }

    pub fn get_node(env: Env, operator: Address) -> Result<NodeRecord, ContractError> {
        get_record(&env, &operator).ok_or(ContractError::NotRegistered)
    }

    pub fn get_capacity(env: Env) -> CapacitySummary {
        CapacitySummary {
            total_cpu_cores: env.storage().instance().get(&DataKey::TotalCpu).unwrap_or(0),
            total_gpu_units: env.storage().instance().get(&DataKey::TotalGpu).unwrap_or(0),
            total_storage_gb: env.storage().instance().get(&DataKey::TotalStorage).unwrap_or(0),
        }
    }
}

fn has_record(env: &Env, operator: &Address) -> bool {
    env.storage().instance().has(&DataKey::NodeRecord(operator.clone()))
}

fn get_record(env: &Env, operator: &Address) -> Option<NodeRecord> {
    env.storage().instance().get(&DataKey::NodeRecord(operator.clone()))
}

fn set_record(env: &Env, operator: &Address, record: &NodeRecord) {
    env.storage().instance().set(&DataKey::NodeRecord(operator.clone()), record);
}

fn append_operator(env: &Env, operator: &Address) {
    let mut operators: Vec<Address> = env.storage().instance().get(&DataKey::RegisteredOperators).unwrap_or(Vec::new(&env));
    operators.push_back(operator.clone());
    env.storage().instance().set(&DataKey::RegisteredOperators, &operators);
}

fn accelerate_capacity(env: &Env, config: &NodeConfig, direction: i8) {
    update_capacity_field(env, DataKey::TotalCpu, config.cpu_cores, direction);
    update_capacity_field(env, DataKey::TotalGpu, config.gpu_units, direction);
    update_capacity_field(env, DataKey::TotalStorage, config.storage_gb, direction);
}

fn adjust_capacity(env: &Env, previous: &NodeConfig, updated: &NodeConfig) {
    if previous.cpu_cores != updated.cpu_cores {
        let delta = updated.cpu_cores as i64 - previous.cpu_cores as i64;
        update_capacity_field(env, DataKey::TotalCpu, delta.unsigned_abs() as u32, if delta >= 0 { 1 } else { -1 });
    }
    if previous.gpu_units != updated.gpu_units {
        let delta = updated.gpu_units as i64 - previous.gpu_units as i64;
        update_capacity_field(env, DataKey::TotalGpu, delta.unsigned_abs() as u32, if delta >= 0 { 1 } else { -1 });
    }
    if previous.storage_gb != updated.storage_gb {
        let delta = updated.storage_gb as i64 - previous.storage_gb as i64;
        update_capacity_field(env, DataKey::TotalStorage, delta.unsigned_abs() as u32, if delta >= 0 { 1 } else { -1 });
    }
}

fn update_capacity_field(env: &Env, key: DataKey, value: u32, direction: i8) {
    let current: u32 = env.storage().instance().get(&key).unwrap_or(0);
    let next = if direction >= 0 {
        current.saturating_add(value)
    } else {
        current.saturating_sub(value)
    };
    env.storage().instance().set(&key, &next);
}

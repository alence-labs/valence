#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[cfg(feature = "soroban")]
use soroban_sdk::{contracttype, Address};

#[cfg(feature = "soroban")]
#[contracttype]
#[derive(Clone)]
pub struct NodeConfig {
    pub cpu_cores: u32,
    pub gpu_units: u32,
    pub storage_gb: u32,
}

#[cfg(not(feature = "soroban"))]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "soroban"))]
#[derive(Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub cpu_cores: u32,
    pub gpu_units: u32,
    pub storage_gb: u32,
}

#[cfg(feature = "soroban")]
#[contracttype]
#[derive(Clone)]
pub struct NodeRecord {
    pub operator: Address,
    pub config: NodeConfig,
    pub stake: i128,
    pub active: bool,
    pub reward_points: u32,
}

#[cfg(not(feature = "soroban"))]
#[derive(Clone, Serialize, Deserialize)]
pub struct NodeRecord {
    pub operator: String,
    pub config: NodeConfig,
    pub stake: i128,
    pub active: bool,
    pub reward_points: u32,
}

#[cfg(feature = "soroban")]
#[contracttype]
#[derive(Clone)]
pub struct CapacitySummary {
    pub total_cpu_cores: u32,
    pub total_gpu_units: u32,
    pub total_storage_gb: u32,
}

#[cfg(not(feature = "soroban"))]
#[derive(Clone, Serialize, Deserialize)]
pub struct CapacitySummary {
    pub total_cpu_cores: u32,
    pub total_gpu_units: u32,
    pub total_storage_gb: u32,
}

impl CapacitySummary {
    pub fn empty() -> Self {
        CapacitySummary {
            total_cpu_cores: 0,
            total_gpu_units: 0,
            total_storage_gb: 0,
        }
    }
}

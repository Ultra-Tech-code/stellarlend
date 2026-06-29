use soroban_sdk::{contracterror, contracttype, Address, String, Val, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MigrationError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidProtocol = 4,
    MigrationFailed = 5,
    RateLimitExceeded = 6,
    BridgeError = 7,
    DeadlineExceeded = 8,
    InsufficientFunds = 9,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProtocolType {
    StellarOther,
    CrossChainBridge,
    AaveMock,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationStatus {
    Pending,
    Completed,
    Failed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationRecord {
    pub user: Address,
    pub protocol: ProtocolType,
    pub asset: Address,
    pub amount: i128,
    pub status: MigrationStatus,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Config,
    Migration(u64),
    UserMigrations(Address),
    NextMigrationId,
    Analytics,
    Admin,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationConfig {
    pub lending_contract: Address,
    pub bridge_contract: Address,
    pub rate_limit_per_ledger: u32,
    pub migration_deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationAnalytics {
    pub total_migrated_value: i128,
    pub total_users: u32,
    pub successful_migrations: u32,
    pub failed_migrations: u32,
}

pub mod contract_terms;

use serde::Deserialize;

use self::contract_terms::ContractTerms;

/// Type of contract.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

/// Contract details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// ID of the contract.
    pub id: String,
    /// The symbol of the faction that this contract is for.
    pub faction_symbol: String,
    /// Type of contract.
    #[serde(rename = "type")]
    pub contract_type: ContractType,
    /// The terms to fulfill the contract.
    pub terms: ContractTerms,
    /// Whether the contract has been accepted by the agent
    pub accepted: bool,
    /// Whether the contract has been fulfilled
    pub fulfilled: bool,
    /// Deprecated in favor of deadlineToAccept
    #[warn(deprecated)]
    pub expiration: String,
    /// The time at which the contract is no longer available to be accepted
    pub deadline_to_accept: Option<chrono::DateTime<chrono::Utc>>,
}

impl PartialEq for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Contract {}
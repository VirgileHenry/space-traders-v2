pub mod contract_deliver_good;
pub mod contract_payments;

use serde::Deserialize;

use self::{
    contract_payments::ContractPayment,
    contract_deliver_good::ContractDeliverGood
};

/// Terms of the contract needed to fulfill it.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    /// The deadline for the contract.
    pub deadline: chrono::DateTime<chrono::Utc>,
    /// Payments for the contract.
    pub payment: ContractPayment,
    /// The cargo that needs to be delivered to fulfill the contract.
    pub deliver: Option<Vec<ContractDeliverGood>>,
}
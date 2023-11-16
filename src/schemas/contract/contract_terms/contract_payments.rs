use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractPayment {
    /// The amount of credits received up front for accepting the contract.
    pub on_accepted: i64,
    /// The amount of credits received when the contract is fulfilled.
    pub on_fulfilled: i64,
}
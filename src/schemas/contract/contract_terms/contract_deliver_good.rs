use serde::Deserialize;

/// The details of a delivery contract. Includes the type of good, units needed, and the destination.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeliverGood {
    /// The symbol of the trade good to deliver.
    pub trade_symbol: String,
    /// The destination where goods need to be delivered.
    pub destination_symbol: String,
    /// The number of units that need to be delivered on this contract.
    pub units_required: i64,
    /// The number of units fulfilled on this contract.
    pub units_fulfilled: i64,
}
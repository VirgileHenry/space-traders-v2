use serde::Deserialize;
use crate::utils::date_time_string::DateTimeString;

/// Details of the ship's fuel tanks including how much fuel was consumed during the last transit or action.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipFuel {
    /// The current amount of fuel in the ship's tanks.
    pub current: u64,
    /// The maximum amount of fuel the ship's tanks can hold.
    pub capacity: u64,
    /// An object that only shows up when an action has consumed fuel in the process. Shows the fuel consumption data.
    pub consumed: Option<ShipConsumedFuel>,
}

/// An object that only shows up when an action has consumed fuel in the process. Shows the fuel consumption data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipConsumedFuel {
    /// The amount of fuel consumed by the most recent transit or action.
    pub amount: u64,
    /// The time at which the fuel was consumed.
    pub timestamp: DateTimeString,
}
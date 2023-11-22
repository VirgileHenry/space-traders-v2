
use std::num::NonZeroU64;

use serde::{Serialize, Deserialize};
use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::wrapper::DataWrapper,
    error::server_error::SpaceTraderError,
    schemas::{
        ship::ship_fuel::ShipFuel,
        agent::Agent,
        market::market_transaction::MarketTransaction
    }
};

/// Struct about how to refuel the ship.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefuelShipInfo {
    /// The amount of fuel to fill in the ship's tanks. When not specified, the ship will be refueled to its maximum fuel capacity. If the amount specified is greater than the ship's remaining capacity, the ship will only be refueled to its maximum fuel capacity. The amount specified is not in market units but in ship fuel units.
    pub units: Option<NonZeroU64>,
    /// Wether to use the FUEL thats in your cargo or not. Default: false
    pub from_cargo: Option<bool>,
}

impl Default for RefuelShipInfo {
    fn default() -> Self {
        RefuelShipInfo { units: None, from_cargo: None }
    }
}

/// Struct about how to refuel the ship.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefuelShipResponse {
    /// Agent details.
    pub agent: Agent,
    /// Details of the ship's fuel tanks including how much fuel was consumed during the last transit or action.
    pub fuel: ShipFuel,
    /// Result of a transaction with a market.
    pub transaction: MarketTransaction,
}

impl SpaceTradersClient<Authenticated> {
    /// Refuel your ship by buying fuel from the local market.
    /// 
    /// Requires the ship to be docked in a waypoint that has the Marketplace trait, and the market must be selling fuel in order to refuel.
    ///     
    /// Each fuel bought from the market replenishes 100 units in your ship's fuel.
    ///     
    /// Ships will always be refuel to their frame's maximum fuel capacity when using this action.
    pub async fn refuel_ship(&self, ship_symbol: &str, refuel_info: &RefuelShipInfo) -> Result<RefuelShipResponse, crate::error::Error> {
        let body = serde_json::to_value(refuel_info)?;
        let response = self.post(&format!("my/ships/{ship_symbol}/refuel"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<RefuelShipResponse>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <SpaceTraderError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }


}
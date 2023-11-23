
use serde::Deserialize;
use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::wrapper::{DataWrapper, ErrorWrapper},
    error::server_error::SpaceTraderError,
    schemas::{
        agent::Agent,
        ship::{Ship, ship_type::ShipType},
        shipyard::shipyard_transaction::ShipyardTransaction,
    }
};

/// Result struct when purchasing a ship.
/// Contains the agent, ship and transaction.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipPurchaseResult {
    /// Agent details.
    pub agent: Agent,
    /// Ship details.
    pub ship: Ship,
    /// Results of a transaction with a shipyard.
    pub transaction: ShipyardTransaction,
}

impl SpaceTradersClient<Authenticated> {
    ///Purchase a ship from a Shipyard. In order to use this function, a ship under your agent's ownership must be in a waypoint that has the Shipyard trait, and the Shipyard must sell the type of the desired ship.
    ///
    /// Shipyards typically offer ship types, which are predefined templates of ships that have dedicated roles. A template comes with a preset of an engine, a reactor, and a frame. It may also include a few modules and mounts.
    pub async fn purchase_ship(&self, ship_prefab: ShipType, at_waypoint: &str) -> Result<ShipPurchaseResult, crate::error::Error> {
        let body = serde_json::json!({
            "shipType": ship_prefab,
            "waypointSymbol": at_waypoint,
        });
        let response = self.post("my/ships")
            .json(&body) 
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipPurchaseResult>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ErrorWrapper<SpaceTraderError>>::deserialize(json)?.inner(); 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

}
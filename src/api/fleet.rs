pub mod chart;
pub mod cooldown;
pub mod refining;
pub mod survey;

use serde::Deserialize;

use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::{
        pagination::page_limit_and_index,
        wrapper::{
            PaginationWrapper,
            DataWrapper
        }
    },
    error::server_error::ServerError,
    schemas::{
        agent::Agent,
        ship::{Ship, ship_type::ShipType, ship_cargo::ShipCargo, ship_nav::ShipNav},
        market::market_transaction::MarketTransaction,
        meta::Meta,
    }
};

/// Result struct when purchasing a ship.
/// Contains the agent, ship and transaction.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipPurchaseResult {
    pub agent: Agent,
    pub ship: Ship,
    pub transaction: MarketTransaction,
}

impl SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all of ships under your agent's ownership.
    pub async fn list_ships(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Ship>, Meta), crate::error::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("my/ships")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<PaginationWrapper::<Ship>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ServerError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

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
                let server_error = <ServerError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Retrieve the details of a ship under your agent's ownership.
    pub async fn get_ship(&self, ship_symbol: &str) -> Result<Ship, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Ship>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ServerError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Retrieve the cargo of a ship under your agent's ownership.
    pub async fn get_ship_cargo(&self, ship_symbol: &str) -> Result<ShipCargo, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/cargo"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipCargo>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ServerError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Attempt to move your ship into orbit at its current location. The request will only succeed if your ship is capable of moving into orbit at the time of the request.
    /// 
    /// Orbiting ships are able to do actions that require the ship to be above surface such as navigating or extracting, but cannot access elements in their current waypoint, such as the market or a shipyard.
    /// 
    /// The endpoint is idempotent - successive calls will succeed even if the ship is already in orbit.
    pub async fn orbit_ship(&self, ship_symbol: &str) -> Result<ShipNav, crate::error::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/orbit"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipNav>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ServerError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Attempt to dock your ship at its current location. Docking will only succeed if your ship is capable of docking at the time of the request.
    ///
    /// Docked ships can access elements in their current location, such as the market or a shipyard, but cannot do actions that require the ship to be above surface such as navigating or extracting.
    ///
    /// The endpoint is idempotent - successive calls will succeed even if the ship is already docked.
    pub async fn dock_ship(&self, ship_symbol: &str) -> Result<ShipNav, crate::error::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/dock"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipNav>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ServerError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    


}
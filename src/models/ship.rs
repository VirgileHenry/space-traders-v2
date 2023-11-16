pub mod chart;
pub mod cooldown;
pub mod crew;
pub mod engine;
pub mod frame;
pub mod fuel;
pub mod modules;
pub mod mount;
pub mod nav;
pub mod reactor;
pub mod refining;
pub mod registration;
pub mod requirements;
pub mod survey;

use serde::{Serialize, Deserialize};

use crate::{
    client::Authenticated,
    utils::{
        pagination::page_limit_and_index,
        wrapper::{
            PaginationWrapper,
            DataWrapper
        }
    }, error::server_error::ServerError
};

use self::nav::Nav;

use super::{
    cooldown::Cooldown,
    cargo::Cargo,
    meta::Meta, agent::Agent,
    transaction::Transaction,
};

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipPrefab {
    ShipProbe,
    ShipMiningDrone,
    ShipSiphonDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
    ShipSurveyor,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    symbol: String,
    registration: self::registration::Registration,
    nav: self::nav::Nav,
    crew: self::crew::Crew,
    frame: self::frame::Frame,
    reactor: self::reactor::Reactor,
    engine: self::engine::Engine,
    cooldown: Cooldown,
    modules: Vec<self::modules::Module>,
    mounts: Vec<self::mount::Mount>,
    cargo: Cargo,
    fuel: self::fuel::Fuel,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipPurchaseResult {
    agent: Agent,
    ship: Ship,
    transaction: Transaction,
}

impl crate::SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all of ships under your agent's ownership.
    pub async fn list_ships(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Ship>, Meta), crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    ///Purchase a ship from a Shipyard. In order to use this function, a ship under your agent's ownership must be in a waypoint that has the Shipyard trait, and the Shipyard must sell the type of the desired ship.
    ///
    /// Shipyards typically offer ship types, which are predefined templates of ships that have dedicated roles. A template comes with a preset of an engine, a reactor, and a frame. It may also include a few modules and mounts.
    pub async fn purchase_ship(&self, ship_prefab: ShipPrefab, at_waypoint: &str) -> Result<ShipPurchaseResult, crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Retrieve the details of a ship under your agent's ownership.
    pub async fn get_ship(&self, ship_symbol: &str) -> Result<Ship, crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Retrieve the cargo of a ship under your agent's ownership.
    pub async fn get_ship_cargo(&self, ship_symbol: &str) -> Result<Cargo, crate::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/cargo"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Cargo>>::deserialize(json)?.inner())
            }
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Attempt to move your ship into orbit at its current location. The request will only succeed if your ship is capable of moving into orbit at the time of the request.
    /// 
    /// Orbiting ships are able to do actions that require the ship to be above surface such as navigating or extracting, but cannot access elements in their current waypoint, such as the market or a shipyard.
    /// 
    /// The endpoint is idempotent - successive calls will succeed even if the ship is already in orbit.
    pub async fn orbit_ship(&self, ship_symbol: &str) -> Result<Nav, crate::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/orbit"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Nav>>::deserialize(json)?.inner())
            }
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Attempt to dock your ship at its current location. Docking will only succeed if your ship is capable of docking at the time of the request.
    ///
    /// Docked ships can access elements in their current location, such as the market or a shipyard, but cannot do actions that require the ship to be above surface such as navigating or extracting.
    ///
    /// The endpoint is idempotent - successive calls will succeed even if the ship is already docked.
    pub async fn dock_ship(&self, ship_symbol: &str) -> Result<Nav, crate::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/dock"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Nav>>::deserialize(json)?.inner())
            }
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    


}
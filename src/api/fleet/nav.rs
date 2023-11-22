use serde::Deserialize;
use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::wrapper::DataWrapper,
    error::server_error::SpaceTraderError,
    schemas::{
        ship::{ship_nav::{ShipNav, ship_nav_flight_mode::ShipNavFlightMode}, ship_fuel::ShipFuel},
        cooldown::Cooldown,
        market::market_transaction::MarketTransaction
    },
};

/// Wrapper around a nav, cooldown, transaction.
/// This is returned from a ship jump.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JumpResult {
    /// The navigation information of the ship.
    pub nav: ShipNav,
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub cooldown: Cooldown,
    /// Result of a transaction with a market.
    pub transaction: MarketTransaction,
}

/// The successful transit information including the route details and changes to ship fuel. The route includes the expected time of arrival.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NavResult {
    /// Details of the ship's fuel tanks including how much fuel was consumed during the last transit or action.
    pub fuel: ShipFuel,
    /// The navigation information of the ship.
    pub nav: ShipNav,
}

impl SpaceTradersClient<Authenticated> {
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
                let server_error = <SpaceTraderError>::deserialize(json)?; 
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
                let server_error = <SpaceTraderError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Jump your ship instantly to a target connected waypoint. The ship must be in orbit to execute a jump.
    /// 
    /// A unit of antimatter is purchased and consumed from the market when jumping. The price of antimatter is determined by the market and is subject to change. A ship can only jump to connected waypoints
    pub async fn jump_ship(&self, ship_symbol: &str, destination_waypoint_symbol: &str) -> Result<JumpResult, crate::error::Error> {
        let body = serde_json::json!({
            "waypointSymbol": destination_waypoint_symbol,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/jump"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<JumpResult>>::deserialize(json)?.inner())
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

    /// Navigate to a target destination. The ship must be in orbit to use this function. The destination waypoint must be within the same system as the ship's current location. Navigating will consume the necessary fuel from the ship's manifest based on the distance to the target waypoint.
    /// 
    /// The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at it's destination.
    /// 
    /// To travel between systems, see the ship's Warp or Jump actions.
    pub async fn navigate_ship(&self, ship_symbol: &str, destination_waypoint_symbol: &str) -> Result<NavResult, crate::error::Error> {
        let body = serde_json::json!({
            "waypointSymbol": destination_waypoint_symbol,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/navigate"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<NavResult>>::deserialize(json)?.inner())
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

    /// Update the nav configuration of a ship.
    /// 
    /// Currently only supports configuring the Flight Mode of the ship, which affects its speed and fuel consumption.
    pub async fn patch_ship_nav(&self, ship_symbol: &str, new_flight_mode: ShipNavFlightMode) -> Result<ShipNav, crate::error::Error> {
        let body = serde_json::json!({
            "flightMode": new_flight_mode,
        });
        let response = self.patch(&format!("my/ships/{ship_symbol}/nav"))
            .json(&body)
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
                let server_error = <SpaceTraderError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get the current nav status of a ship.
    pub async fn get_ship_nav(&self, ship_symbol: &str) -> Result<ShipNav, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/nav"))
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
                let server_error = <SpaceTraderError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Warp your ship to a target destination in another system. The ship must be in orbit to use this function and must have the Warp Drive module installed. Warping will consume the necessary fuel from the ship's manifest.
    ///
    /// The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at its destination.
    pub async fn warp_ship(&self, ship_symbol: &str, destination_waypoint_symbol: &str) -> Result<NavResult, crate::error::Error> {
        let body = serde_json::json!({
            "waypointSymbol": destination_waypoint_symbol,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/warp"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<NavResult>>::deserialize(json)?.inner())
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
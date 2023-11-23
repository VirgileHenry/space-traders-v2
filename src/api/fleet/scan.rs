
use serde::Deserialize;
use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::wrapper::{DataWrapper, ErrorWrapper},
    error::server_error::SpaceTraderError,
    schemas::{
        cooldown::Cooldown,
        scanned::{
            scanned_system::ScannedSystem,
            scanned_waypoint::ScannedWaypoint,
            scanned_ship::ScannedShip
        }
    },
};


/// Wrapper around a cooldown, and a list of scanned systems.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemsScanResult {
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub cooldown: Cooldown,
    /// List of scanned systems.
    pub systems: Vec<ScannedSystem>,
}

/// Wrapper around a cooldown, and a list of scanned waypoints.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaypointsScanResult {
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub cooldown: Cooldown,
    /// List of scanned waypoints.
    pub waypoints: Vec<ScannedWaypoint>,
}

/// Wrapper around a cooldown, and a list of scanned ships.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipsScanResult {
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub cooldown: Cooldown,
    /// List of scanned ships.
    pub ships: Vec<ScannedShip>,
}



impl SpaceTradersClient<Authenticated> {
    /// Scan for nearby systems, retrieving information on the systems' distance from the ship and their waypoints. Requires a ship to have the Sensor Array mount installed to use.
    /// 
    /// The ship will enter a cooldown after using this function, during which it cannot execute certain actions.
    pub async fn scan_systems(&self, ship_symbol: &str) -> Result<SystemsScanResult, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/scan/systems"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<SystemsScanResult>>::deserialize(json)?.inner())
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

    /// Scan for nearby waypoints, retrieving detailed information on each waypoint in range. Scanning uncharted waypoints will allow you to ignore their uncharted state and will list the waypoints' traits.
    ///     
    /// Requires a ship to have the Sensor Array mount installed to use.
    ///     
    /// The ship will enter a cooldown after using this function, during which it cannot execute certain actions.
    pub async fn scan_waypoint(&self, ship_symbol: &str) -> Result<WaypointsScanResult, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/scan/waypoints"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<WaypointsScanResult>>::deserialize(json)?.inner())
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

    ///Scan for nearby ships, retrieving information for all ships in range.
    ///
    ///Requires a ship to have the Sensor Array mount installed to use.
    ///
    ///The ship will enter a cooldown after using this function, during which it cannot execute certain actions.
    pub async fn scan_ships(&self, ship_symbol: &str) -> Result<ShipsScanResult, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/scan/ships"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipsScanResult>>::deserialize(json)?.inner())
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
use serde::Deserialize;
use crate::{
    schemas::{
        ship::{
            ship_mount::{ShipMount, ShipMountType},
            ship_cargo::ShipCargo
        },
        agent::Agent,
        market::market_transaction::MarketTransaction
    },
    client::{
        SpaceTradersClient,
        Authenticated
    },
    utils::wrapper::{ArrayWrapper, DataWrapper},
    error::server_error::SpaceTraderError
};

/// Wrapper around a agent, mounts, cargo and transaction.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MountInstallationResult {
    /// Agent details.
    pub agent: Agent,
    /// List of installed mounts after the installation of the new mount.
    pub mounts: Vec<ShipMount>,
    /// Ship cargo details.
    pub cargo: ShipCargo,
    /// Result of a transaction for a ship modification, such as installing a mount or a module.
    pub transaction: MarketTransaction,
}

impl SpaceTradersClient<Authenticated> {
    /// Get the mounts installed on a ship.
    pub async fn get_mounts(&self, ship_symbol: &str) -> Result<Vec<ShipMount>, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/mounts"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<ArrayWrapper::<ShipMount>>::deserialize(json)?.inner())
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

    /// Install a mount on a ship.
    /// 
    /// In order to install a mount, the ship must be docked and located in a waypoint that has a Shipyard trait. The ship also must have the mount to install in its cargo hold.
    /// 
    /// An installation fee will be deduced by the Shipyard for installing the mount on the ship.
    pub async fn install_mounts(&self, ship_symbol: &str, mount: ShipMountType) -> Result<MountInstallationResult, crate::error::Error> {
        let body = serde_json::json!({
            "symbol": mount,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/mounts/install"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<MountInstallationResult>>::deserialize(json)?.inner())
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

    /// Remove a mount from a ship.
    /// 
    /// The ship must be docked in a waypoint that has the Shipyard trait, and must have the desired mount that it wish to remove installed.
    /// 
    /// A removal fee will be deduced from the agent by the Shipyard.
    pub async fn remove_mounts(&self, ship_symbol: &str, mount: ShipMountType) -> Result<MountInstallationResult, crate::error::Error> {
        let body = serde_json::json!({
            "symbol": mount,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/mounts/remove"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<MountInstallationResult>>::deserialize(json)?.inner())
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



pub mod waypoint;

use serde::Deserialize;
use crate::{
    client::{
        SpaceTradersClient,
        Anonymous,
        Authenticated
    },
    schemas::{system::System, meta::Meta},
    utils::wrapper::{PaginationWrapper, DataWrapper},
    error::server_error::SpaceTraderError
};

impl SpaceTradersClient<Anonymous> {
    /// Return a paginated list of all systems.
    pub async fn list_systems(&self) -> Result<(Vec<System>, Meta), crate::error::Error> {
        let response = self.get("systems").send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(PaginationWrapper::<System>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get the details of a system.
    pub async fn get_systems(&self, system_symbol: &str) -> Result<System, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<System>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }
}


impl SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all systems.
    pub async fn list_systems(&self) -> Result<(Vec<System>, Meta), crate::error::Error> {
        let response = self.get("systems").send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(PaginationWrapper::<System>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get the details of a system.
    pub async fn get_systems(&self, system_symbol: &str) -> Result<System, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<System>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }
}
use serde::Deserialize;

use crate::{
    client::{
        Authenticated,
        Anonymous,
        SpaceTradersClient
    },
    utils::{
        wrapper::{PaginationWrapper, DataWrapper},
        pagination::page_limit_and_index
    },
    error::server_error::ServerError,
    schemas::{faction::Faction, meta::Meta}
};


impl SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all the factions in the game.
    pub async fn list_factions(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Faction>, Meta), crate::error::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("factions")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<PaginationWrapper::<Faction>>::deserialize(json)?.inner())
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

    /// View the details of a faction.
    pub async fn get_faction(&self, faction_symbol: &str) -> Result<Faction, crate::error::Error> {
        let response = self.get(&format!("factions/{faction_symbol}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Faction>>::deserialize(json)?.inner())
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

impl SpaceTradersClient<Anonymous> {
    /// Return a paginated list of all the factions in the game.
    pub async fn list_factions(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Faction>, Meta), crate::error::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("factions")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<PaginationWrapper::<Faction>>::deserialize(json)?.inner())
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

    /// View the details of a faction.
    pub async fn get_faction(&self, faction_symbol: &str) -> Result<Faction, crate::error::Error> {
        let response = self.get(&format!("factions/{faction_symbol}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Faction>>::deserialize(json)?.inner())
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
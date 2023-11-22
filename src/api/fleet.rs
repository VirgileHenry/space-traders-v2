pub mod cargo;
pub mod chart;
pub mod cooldown;
pub mod mounts;
pub mod nav;
pub mod purchase;
pub mod refining;
pub mod refuel;
pub mod resources;
pub mod survey;
pub mod scan;

use serde::Deserialize;
use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::{
        pagination::page_limit_and_index,
        wrapper::{PaginationWrapper, DataWrapper}
    },
    error::server_error::SpaceTraderError,
    schemas::{ship::Ship, meta::Meta, contract::Contract}
};

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
                let server_error = <SpaceTraderError>::deserialize(json)?; 
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
                let server_error = <SpaceTraderError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Negotiate a new contract with the HQ.
    /// 
    /// In order to negotiate a new contract, an agent must not have ongoing or offered contracts over the allowed maximum amount. Currently the maximum contracts an agent can have at a time is 1.
    /// 
    /// Once a contract is negotiated, it is added to the list of contracts offered to the agent, which the agent can then accept.
    /// 
    /// The ship must be present at any waypoint with a faction present to negotiate a contract with that faction.
    pub async fn negotiate_contract(&self, ship_symbol: &str) -> Result<Contract, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/negotiate/contract"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Contract>>::deserialize(json)?.inner())
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
use serde::Deserialize;
use crate::{
    utils::{
        wrapper::{DataWrapper, PaginationWrapper, ErrorWrapper},
        pagination::page_limit_and_index
    },
    client::{Authenticated, Anonymous},
    error::server_error::SpaceTraderError,
    schemas::{agent::Agent,meta::Meta},
};


impl crate::client::SpaceTradersClient<Authenticated> {
    /// Fetch your agent's details.
    pub async fn get_agent(&self) -> Result<Agent, crate::error::Error> {
        let response = self.get("my/agent")
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Agent>>::deserialize(json)?.inner())
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

    /// Fetch agents details.
    pub async fn list_agents(&self, page_limit: Option<u64>, page_index: Option<u64>) -> Result<(Vec<Agent>, Meta), crate::error::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("agents")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<PaginationWrapper::<Agent>>::deserialize(json)?.inner())
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

    /// Fetch agent details.
    pub async fn get_public_agent(&self, agent_symbol: &str) -> Result<Agent, crate::error::Error> {
        let response = self.get(&format!("agents/{agent_symbol}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Agent>>::deserialize(json)?.inner())
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

impl crate::client::SpaceTradersClient<Anonymous> {
    /// Fetch agents details.
    pub async fn list_agents(&self, page_limit: Option<u64>, page_index: Option<u64>) -> Result<(Vec<Agent>, Meta), crate::error::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("agents")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<PaginationWrapper::<Agent>>::deserialize(json)?.inner())
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

    /// Fetch agent details.
    pub async fn get_public_agent(&self, agent_symbol: &str) -> Result<Agent, crate::error::Error> {
        let response = self.get(&format!("agents/{agent_symbol}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Agent>>::deserialize(json)?.inner())
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

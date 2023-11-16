use serde::Deserialize;

use crate::{
    client::{
        Authenticated,
        SpaceTradersClient
    },
    utils::{
        wrapper::{
            PaginationWrapper,
            DataWrapper
        },
        pagination::page_limit_and_index
    },
    error::server_error::ServerError,
    schemas::{
        contract::Contract,
        meta::Meta, agent::Agent, ship::ship_cargo::ShipCargo
    }
};

/// Wrapper around an agent and a contract.
#[derive(Deserialize, Clone, Debug)]
pub struct AgentAndContract {
    pub agent: Agent,
    pub contract: Contract,
}

/// Wrapper around a contract and a cargo.
#[derive(Deserialize, Clone, Debug)]
pub struct ContractAndCargo {
    pub contract: Contract,
    pub cargo: ShipCargo,
}


impl SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all your contracts.
    pub async fn list_contracts(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Contract>, Meta), crate::error::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("my/contracts")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<PaginationWrapper::<Contract>>::deserialize(json)?.inner())
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

    /// Get the details of a contract by ID.
    pub async fn get_contract(&self, contract_id: &str) -> Result<Contract, crate::error::Error> {
        let response = self.get(&format!("my/contracts/{contract_id}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Contract>>::deserialize(json)?.inner())
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

    /// Accept a contract by ID.
    /// 
    /// You can only accept contracts that were offered to you,
    /// were not accepted yet, and whose deadlines has not passed yet.
    pub async fn accept_contract(&self, contract_id: &str) -> Result<AgentAndContract, crate::error::Error> {
        let response = self.post(&format!("my/contracts/{contract_id}/accept"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<AgentAndContract>>::deserialize(json)?.inner())
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

    /// Deliver cargo to a contract.
    /// In order to use this API, a ship must be at the delivery location
    /// (denoted in the delivery terms as destinationSymbol of a contract)
    /// and must have a number of units of a good required by this contract in its cargo.
    /// Cargo that was delivered will be removed from the ship's cargo.
    pub async fn deliver_cargo_to_contract(&self, contract_id: &str, ship_symbol: &str, trade_symbol: &str, units: u64) -> Result<ContractAndCargo, crate::error::Error> {
        let body = serde_json::json!({
            "shipSymbol": ship_symbol,
            "tradeSymbol": trade_symbol,
            "units": units
        });
        let response = self.post(&format!("my/contracts/{contract_id}/deliver"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ContractAndCargo>>::deserialize(json)?.inner())
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

    /// Fulfill a contract.
    /// Can only be used on contracts that have all of their delivery terms fulfilled.
    pub async fn fulfill_contract(&self, contract_id: &str) -> Result<AgentAndContract, crate::error::Error> {
        let response = self.post(&format!("my/contracts/{contract_id}/fulfill"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<AgentAndContract>>::deserialize(json)?.inner())
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


#[cfg(test)]
mod test {
    use crate::client::TEST_AGENT_TOKEN;

    #[tokio::test]
    async fn test_agent() {
        let client = crate::client::SpaceTradersClient::new_with_auth(TEST_AGENT_TOKEN);
        let contracts = client.list_contracts(None, None).await;
        println!("{contracts:?}");
        let contract = client.get_contract("cloyrd0gs1edps60cl4z96536").await;
        println!("{contract:?}")
    }
}
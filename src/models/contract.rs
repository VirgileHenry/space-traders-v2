use serde::Deserialize;

use crate::{
    client::Authenticated,
    utils::{
        wrapper::{
            PaginationWrapper,
            DataWrapper
        }, pagination::page_limit_and_index
    }, error::server_error::ServerError
};

use super::meta::Meta;



#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    deadline: String,
    payment: ContractPayment,
    deliver: Vec<ContractCargo>,
}


/// Wrapper around a contract and a cargo.
/// Value returned by deliver_cargo_to_contract().
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractAndCargo {
    contract: Contract,
    cargo: super::cargo::Cargo,
}

/// Wrapper around a agent and a contract.
/// Value returned by fulfill_contract().
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentAndContract {
    agent: super::agent::Agent,
    contract: Contract,
}

impl crate::SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all your contracts.
    pub async fn list_contracts(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Contract>, Meta), crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Get the details of a contract by ID.
    pub async fn get_contract(&self, contract_id: &str) -> Result<Contract, crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Accept a contract by ID.
    /// 
    /// You can only accept contracts that were offered to you,
    /// were not accepted yet, and whose deadlines has not passed yet.
    pub async fn accept_contract(&self, contract_id: &str) -> Result<AgentAndContract, crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Deliver cargo to a contract.
    /// In order to use this API, a ship must be at the delivery location
    /// (denoted in the delivery terms as destinationSymbol of a contract)
    /// and must have a number of units of a good required by this contract in its cargo.
    /// Cargo that was delivered will be removed from the ship's cargo.
    pub async fn deliver_cargo_to_contract(&self, contract_id: &str, ship_symbol: &str, trade_symbol: &str, units: u64) -> Result<ContractAndCargo, crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Fulfill a contract.
    /// Can only be used on contracts that have all of their delivery terms fulfilled.
    pub async fn fulfill_contract(&self, contract_id: &str) -> Result<AgentAndContract, crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
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
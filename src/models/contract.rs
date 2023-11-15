use serde::Deserialize;

use crate::{
    client::Authenticated,
    utils::{
        deserialize::deserialize,
        wrapper::{
            DataAndMetaWrapper,
            DataWrapper
        }
    }
};

use super::meta::Meta;

/// Type of contract.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

/// Contract details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    id: String,
    faction_symbol: String,
    #[serde(rename = "type")]
    contract_type: ContractType,
    terms: ContractTerms,
    accepted: bool,
    fulfilled: bool,
    #[warn(deprecated)]
    expiration: String,
    deadline_to_accept: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    deadline: String,
    payment: ContractPayment,
    deliver: Vec<ContractCargo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractPayment {
    on_accepted: i64,
    on_fulfilled: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractCargo {
    trade_symbol: String,
    destination_symbol: String,
    units_required: i64,
    units_fulfilled: i64,
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
    pub async fn list_contracts(&self) -> Result<(Vec<Contract>, Meta), crate::Error> {
        let request = self.get("my/contracts")
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataAndMetaWrapper::<Contract>>(json)?.inner())
    }

    /// Get the details of a contract by ID.
    pub async fn get_contract(&self, contract_id: &str) -> Result<Contract, crate::Error> {
        let request = self.get(&format!("my/contracts/{contract_id}"))
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<Contract>>(json)?.inner())
    }

    /// Accept a contract by ID.
    /// You can only accept contracts that were offered to you,
    /// were not accepted yet, and whose deadlines has not passed yet.
    pub async fn accept_contract(&self, contract_id: &str) -> Result<Contract, crate::Error> {
        let request = self.post(&format!("my/contracts/{contract_id}/accept"))
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<Contract>>(json)?.inner())
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
        let request = self.post(&format!("my/contracts/{contract_id}/deliver"))
            .json(&body)
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<ContractAndCargo>>(json)?.inner())
    }

    /// Fulfill a contract.
    /// Can only be used on contracts that have all of their delivery terms fulfilled.
    pub async fn fulfill_contract(&self, contract_id: &str) -> Result<AgentAndContract, crate::Error> {
        let request = self.post(&format!("my/contracts/{contract_id}/fulfill"))
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<AgentAndContract>>(json)?.inner())
    }
}


#[cfg(test)]
mod test {
    use crate::client::TEST_AGENT_TOKEN;

    #[tokio::test]
    async fn test_agent() {
        let client = crate::client::SpaceTradersClient::new_with_auth(TEST_AGENT_TOKEN);
        let contracts = client.list_contracts().await;
        println!("{contracts:?}");
        let contract = client.get_contract("cloyrd0gs1edps60cl4z96536").await;
        println!("{contract:?}")
    }
}
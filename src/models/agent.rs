use serde::Deserialize;

use crate::{
    utils::{
        deserialize::deserialize,
        wrapper::{DataWrapper, DataAndMetaWrapper}
    },
    client::{
        AuthState,
        Authenticated, Anonymous
    }
};

use super::meta::Meta;

/// Represent a in-game agent.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    /// The account id is only set for our own agents ? 
    account_id: Option<String>,
    symbol: String,
    headquarters: String,
    credits: i64,
    starting_faction: String,
    ship_count: u64,
}

impl crate::client::SpaceTradersClient<Authenticated> {
    /// Fetch your agent's details.
    pub async fn get_agent(&self) -> Result<Agent, crate::Error> {
        let request = self.get("my/agent")
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<Agent>>(json)?.inner())
    }

    /// Fetch agents details.
    pub async fn list_agents(&self) -> Result<(Vec<Agent>, Meta), crate::Error> {
        let request = self.get("agents")
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataAndMetaWrapper::<Agent>>(json)?.inner())
    }

    /// Fetch agent details.
    pub async fn get_public_agent(&self, agent_symbol: &str) -> Result<Agent, crate::Error> {
        let request = self.get(&format!("agents/{agent_symbol}"))
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<Agent>>(json)?.inner())
    }
}

impl crate::client::SpaceTradersClient<Anonymous> {
    /// Fetch agents details.
    pub async fn list_agents(&self) -> Result<(Vec<Agent>, Meta), crate::Error> {
        let request = self.get("agents")
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataAndMetaWrapper::<Agent>>(json)?.inner())
    }

    /// Fetch agent details.
    pub async fn get_public_agent(&self, agent_symbol: &str) -> Result<Agent, crate::Error> {
        let request = self.get(&format!("agents/{agent_symbol}"))
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<Agent>>(json)?.inner())
    }
}

#[cfg(test)]
mod test {
    use crate::client::TEST_AGENT_TOKEN;

    #[tokio::test]
    async fn test_agent() {
        let client = crate::client::SpaceTradersClient::new_with_auth(TEST_AGENT_TOKEN);
        let agent = client.get_agent().await;
        println!("{agent:?}");
        let agents = client.list_agents().await;
        println!("{agents:?}");
        let luciole = client.get_public_agent("LUCIOLE").await;
        println!("{luciole:?}");
    }
}
use serde::Deserialize;
use crate::{utils::deserialize::deserialize, client::{AuthState, SpaceTradersClient, Anonymous, Authenticated}};

use super::{agent::Agent, wrapper::DataWrapper};

/// A registered agent is an agent that have juste been registered.
/// It is a wrapper around an agent and additionnal info regarding creation, like auth token.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredAgent {
    agent: Agent,
    token: String,
    // contract: Contract,
}

impl crate::client::SpaceTradersClient<Anonymous> {
    pub async fn create_agent(&self, email: &str, faction: &str, symbol: &str) -> Result<RegisteredAgent, crate::Error> {
        let body = serde_json::json!({
            "faction": faction,
            "symbol": symbol,
            "email": email
        });
        let request = self.post("register")
            .json(&body)
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<RegisteredAgent>>(json)?.inner())
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_agent() {
        let client = crate::client::SpaceTradersClient::new_anonymous();
        let agent = client.create_agent("virgile.henry0211@gmail.com", "COSMIC", "ROSSIGNOL").await;
        println!("{:?}", agent);
    }
}
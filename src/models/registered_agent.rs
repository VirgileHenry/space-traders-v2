use serde::Deserialize;
use crate::{utils::{deserialize::deserialize, wrapper::DataWrapper}, client::{AuthState, SpaceTradersClient, Anonymous, Authenticated}};

use super::agent::Agent;

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
    /// Creates a new agent and ties it to an account.
    /// The agent symbol must consist of a 3-14 character string,
    /// and will be used to represent your agent.
    /// This symbol will prefix the symbol of every ship you own.
    /// Agent symbols will be cast to all uppercase characters.
    /// This new agent will be tied to a starting faction of your choice,
    /// which determines your starting location, and will be granted an authorization token,
    /// a contract with their starting faction,
    /// a command ship that can fly across space with advanced capabilities,
    /// a small probe ship that can be used for reconnaissance, and 150,000 credits.
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
        let agent = client.create_agent("your.email@here.com", "COSMIC", "YOUR_AGENT_NAME").await;
        println!("{:?}", agent);
    }
}
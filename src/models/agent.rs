use serde::Deserialize;

use crate::{utils::deserialize::deserialize, client::{AuthState, Authenticated}};

use super::wrapper::DataWrapper;

/// Represent a in-game agent.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i64,
    starting_faction: String,
    ship_count: u64,
}

impl crate::client::SpaceTradersClient<Authenticated> {
    pub async fn get_agent(&self) -> Result<Agent, crate::Error> {
        let request = self.get("my/agent")
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
        println!("{:?}", agent);
    }
}
use serde::Deserialize;

use crate::{
    utils::{
        wrapper::{
            DataWrapper,
            PaginationWrapper
        },
        pagination::page_limit_and_index
    },
    client::{
        AuthState,
        Authenticated, Anonymous
    }, error::server_error::ServerError
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Fetch agents details.
    pub async fn list_agents(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Agent>, Meta), crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Fetch agent details.
    pub async fn get_public_agent(&self, agent_symbol: &str) -> Result<Agent, crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }
}

impl crate::client::SpaceTradersClient<Anonymous> {
    /// Fetch agents details.
    pub async fn list_agents(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Agent>, Meta), crate::Error> {
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
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// Fetch agent details.
    pub async fn get_public_agent(&self, agent_symbol: &str) -> Result<Agent, crate::Error> {
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
        let agent = client.get_agent().await;
        println!("{agent:?}");
        let agents = client.list_agents(None, None).await;
        println!("{agents:?}");
        let luciole = client.get_public_agent("LUCIOLE").await;
        println!("{luciole:?}");
    }
}
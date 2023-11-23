use serde::Deserialize;
use crate::{
    utils::wrapper::{DataWrapper, ErrorWrapper},
    client::Anonymous,
    error::server_error::SpaceTraderError,
    schemas::{
        agent::Agent,
        contract::Contract,
        faction::Faction,
        ship::Ship
    },
};

/// A registered agent is an agent that have juste been registered.
/// It is a wrapper around an agent and additionnal info regarding creation, like auth token.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredAgent {
    /// The newly created agent.
    pub agent: Agent,
    /// The secret auth token that this agent auth with. 
    pub token: String,
    /// The starting contract of this agent.
    pub contract: Contract,
    /// The starting faction of this agent.
    pub faction: Faction,
    /// The sole starting ship of this agent.
    pub ship: Ship,
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
    pub async fn create_agent(&self, email: &str, faction: &str, symbol: &str) -> Result<RegisteredAgent, crate::error::Error> {
        let body = serde_json::json!({
            "faction": faction,
            "symbol": symbol,
            "email": email
        });
        let response = self.post("register")
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<RegisteredAgent>>::deserialize(json)?.inner())
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

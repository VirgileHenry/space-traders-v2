use serde::Deserialize;

use crate::{
    client::{
        Authenticated,
        Anonymous
    },
    utils::wrapper::DataWrapper,
    error::server_error::ServerError
};

/// Info about the current server.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatus {
    pub status: String,
    pub version: String,
    pub reset_date: String,
    pub description: String,
    pub stats: ServerStats,
    pub leaderboards: ServerLeaderboards,
    pub server_resets: ServerResets,
    pub announcements: Vec<ServerAnnoucements>,
    pub links: Vec<ServerLinks>,
}

/// Server statistics.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerStats {
    pub agents: u64,
    pub ships: u64,
    pub systems: u64,
    pub waypoints: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerLeaderboards {
    pub most_credits: Vec<MostCreditAgent>,
    pub most_submitted_charts: Vec<MostSubmittedChartsAgent>,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MostCreditAgent {
    pub agent_symbol: String,
    pub credits: i64,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MostSubmittedChartsAgent {
    pub agent_symbol: String,
    pub chart_count: i64,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerResets {
    pub next: String,
    pub frequency: String,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerAnnoucements {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerLinks {
    pub name: String,
    pub url: String,
}

impl crate::client::SpaceTradersClient<Authenticated> {
    /// Return the status of the game server.
    /// This also includes a few global elements, such as announcements,
    /// server reset dates and leaderboards.
    pub async fn get_server_status(&self) -> Result<ServerStatus, crate::error::Error> {
        let response = self.get("")
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ServerStatus>>::deserialize(json)?.inner())
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

impl crate::client::SpaceTradersClient<Anonymous> {
    /// Return the status of the game server.
    /// This also includes a few global elements, such as announcements,
    /// server reset dates and leaderboards.
    pub async fn get_server_status(&self) -> Result<ServerStatus, crate::error::Error> {
        let response = self.get("")
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(ServerStatus::deserialize(json)?)
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

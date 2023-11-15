use serde::Deserialize;

use crate::{client::{Authenticated, Anonymous}, utils::deserialize::deserialize};

/// Info about the current server.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatus {
    status: String,
    version: String,
    reset_date: String,
    description: String,
    stats: ServerStats,
    leaderboards: ServerLeaderboards,
    server_resets: ServerResets,
    announcements: Vec<ServerAnnoucements>,
    links: Vec<ServerLinks>,
}

/// Server statistics.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerStats {
    agents: u64,
    ships: u64,
    systems: u64,
    waypoints: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerLeaderboards {
    most_credits: Vec<MostCreditAgent>,
    most_submitted_charts: Vec<MostSubmittedChartsAgent>,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MostCreditAgent {
    agent_symbol: String,
    credits: i64,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MostSubmittedChartsAgent {
    agent_symbol: String,
    chart_count: i64,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerResets {
    next: String,
    frequency: String,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerAnnoucements {
    title: String,
    body: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerLinks {
    name: String,
    url: String,
}

impl crate::client::SpaceTradersClient<Authenticated> {
    /// Return the status of the game server.
    /// This also includes a few global elements, such as announcements,
    /// server reset dates and leaderboards.
    pub async fn get_server_status(&self) -> Result<ServerStatus, crate::Error> {
        let request = self.get("")
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<ServerStatus>(json)?)
    }
}

impl crate::client::SpaceTradersClient<Anonymous> {
    /// Return the status of the game server.
    /// This also includes a few global elements, such as announcements,
    /// server reset dates and leaderboards.
    pub async fn get_server_status(&self) -> Result<ServerStatus, crate::Error> {
        let request = self.get("")
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<ServerStatus>(json)?)
    }
}

#[cfg(test)]
mod test {
    use crate::client::TEST_AGENT_TOKEN;

    #[tokio::test]
    async fn test_agent() {
        let auth_client = crate::client::SpaceTradersClient::new_with_auth(TEST_AGENT_TOKEN);
        let anon_client = crate::client::SpaceTradersClient::new_anonymous();

        let server_status = auth_client.get_server_status().await;
        println!("{:?}", server_status);
        let server_status = anon_client.get_server_status().await;
        println!("{:?}", server_status);
    }
}
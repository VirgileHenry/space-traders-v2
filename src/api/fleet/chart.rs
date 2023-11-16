use serde::Deserialize;
use crate::{
    schemas::{
        chart::Chart,
        waypoint::Waypoint
    },
    client::{
        SpaceTradersClient,
        Authenticated
    },
    utils::wrapper::DataWrapper,
    error::server_error::ServerError
};

/// Wrapper around a chart and a waypoint.
/// Result of a chart creation.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChartAndWaypoint {
    pub chart: Chart,
    pub waypoint: Waypoint,
}

impl SpaceTradersClient<Authenticated> {
    /// Command a ship to chart the waypoint at its current location.
    /// 
    /// Most waypoints in the universe are uncharted by default. These waypoints have their traits hidden until they have been charted by a ship.
    ///
    /// Charting a waypoint will record your agent as the one who created the chart, and all other agents would also be able to see the waypoint's traits.
    pub async fn create_chart(&self, ship_symbol: &str) -> Result<ChartAndWaypoint, crate::error::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/chart"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ChartAndWaypoint>>::deserialize(json)?.inner())
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



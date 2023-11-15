use serde::Deserialize;

use crate::{
    client::Authenticated,
    utils::wrapper::DataWrapper,
    models::universe::{chart::Chart, waypoint::Waypoint}, error::server_error::ServerError
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChartCreation {
    chart: Chart,
    waypoint: Waypoint,
}

impl crate::client::SpaceTradersClient<Authenticated> {
    /// Command a ship to chart the waypoint at its current location.
    /// 
    /// Most waypoints in the universe are uncharted by default. These waypoints have their traits hidden until they have been charted by a ship.
    ///
    /// Charting a waypoint will record your agent as the one who created the chart, and all other agents would also be able to see the waypoint's traits.
    pub async fn create_chart(&self, ship_symbol: &str) -> Result<ChartCreation, crate::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/chart"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ChartCreation>>::deserialize(json)?.inner())
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



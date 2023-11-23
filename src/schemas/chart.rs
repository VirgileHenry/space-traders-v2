use serde::Deserialize;



/// The chart of a system or waypoint, which makes the location visible to other agents.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    /// The symbol of the waypoint.
    pub waypoint_symbol: Option<String>,
    /// The agent that submitted the chart for this waypoint.
    pub submitted_by: Option<String>,
    /// The time the chart for this waypoint was submitted.
    pub submitted_on: Option<chrono::DateTime<chrono::Utc>>,
}

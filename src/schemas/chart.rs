use serde::Deserialize;

use crate::utils::date_time_string::DateTimeString;

/// The chart of a system or waypoint, which makes the location visible to other agents.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    /// The symbol of the waypoint.
    pub waypoint_symbol: String,
    /// The agent that submitted the chart for this waypoint.
    pub submitted_by: String,
    /// The time the chart for this waypoint was submitted.
    pub submitted_on: DateTimeString,
}

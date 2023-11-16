use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JumpGate {
    /// All the gates that are connected to this waypoint.
    pub connections: Vec<String>,
}
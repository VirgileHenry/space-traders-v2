use serde::Deserialize;

/// The requirements for installation on a ship
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipRequirements {
    /// The amount of power required from the reactor.
    pub power: Option<i64>,
    /// The number of crew required for operation.
    pub crew: Option<i64>,
    /// The number of module slots required for installation.
    pub slots: Option<i64>,
}



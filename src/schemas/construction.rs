pub mod construction_material;

use serde::Deserialize;

use self::construction_material::ConstructionMaterial;


/// The construction details of a waypoint.
#[derive(Deserialize, Debug, Clone)]
pub struct Construction {
    /// The symbol of the waypoint.
    pub symbol: String,
    /// The materials required to construct the waypoint.
    pub materials: Vec<ConstructionMaterial>,
    /// Whether the waypoint has been constructed.
    pub is_complete: bool,
}

impl PartialEq for Construction {
    fn eq(&self, other: &Self) -> bool {
        // there is only one construction per waypoint,
        // so eq on construction is eq on waypoint symbol
        self.symbol == other.symbol
    }
}
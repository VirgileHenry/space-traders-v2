pub mod faction_symbol;
pub mod faction_trait;

use serde::Deserialize;
use self::{
    faction_symbol::FactionSymbol,
    faction_trait::FactionTrait,
};

/// Faction details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    /// Faction symbol.
    pub symbol: FactionSymbol,
    /// Name of the faction.
    pub name: String,
    /// Description of the faction.
    pub description: String,
    /// The waypoint in which the faction's HQ is located in.
    pub headquarters: String,
    /// List of traits that define this faction.
    pub traits: Vec<FactionTrait>,
    /// Whether or not the faction is currently recruiting new agents.
    pub is_recruiting: bool,
}
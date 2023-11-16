use serde::Deserialize;
use crate::schemas::faction::faction_symbol::FactionSymbol;

/// Faction symbol within a system.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemFaction {
    /// The symbol of the faction.
    pub symbol: FactionSymbol
}
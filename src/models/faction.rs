use serde::Deserialize;

use crate::{client::{Authenticated, Anonymous}, utils::{deserialize::deserialize, wrapper::{DataAndMetaWrapper, DataWrapper}, pagination::page_limit_and_index}};

use super::meta::Meta;


/// All existing factions.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionSymbol {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
    Obsidian,
    Aegis,
    United,
    Solitary,
    Cobalt,
    Omega,
    Echo,
    Lords,
    Cult,
    Ancients,
    Shadow,
    Etheral,
}

/// All existing factions.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionTraitType {
    Bureaucratic,
    Secretive,
    Capitalistic,
    Industrious,
    Peaceful,
    Distrustful,
    Welcoming,
    Smugglers,
    Scavangers,
    Rebelious,
    Exiles,
    Pirates,
    Raiders,
    Clan,
    Guild,
    Dominion,
    Fringe,
    Forsaken,
    Isolated,
    Localized,
    Established,
    Notable,
    Dominant,
    Inescapable,
    Innovative,
    Bold,
    Visionary,
    Curious,
    Daring,
    Exploratory,
    Resourceful,
    Flexible,
    Cooperative,
    United,
    Strategic,
    Intelligent,
    ResearchFocused,
    Collaborative,
    Progessive,
    Militaristic,
    TechnologicallyAdvanced,
    Agressive,
    Imperialistic,
    TreasureHunters,
    Dexterious,
    Unpredictible,
    Brutal,
    Fleeting,
    Adaptable,
    SelfSufficient,
    Defensive,
    Proud,
    Diverse,
    Independant,
    SelfInterested,
    Fragmented,
    Commercial,
    FreeMarkets,
    Entrepreneurial,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FactionTrait {
    symbol: FactionTraitType,
    name: String,
    description: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    symbol: FactionSymbol,
    name: String,
    description: String,
    headquarters: String,
    traits: Vec<FactionTrait>,
    is_recruiting: bool,
}

impl crate::SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all the factions in the game.
    pub async fn list_factions(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Faction>, Meta), crate::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let request = self.get("factions")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataAndMetaWrapper::<Faction>>(json)?.inner())
    }

    /// View the details of a faction.
    pub async fn get_faction(&self, faction_symbol: &str) -> Result<Faction, crate::Error> {
        let request = self.get(&format!("factions/{faction_symbol}"))
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<Faction>>(json)?.inner())
    }
}

impl crate::SpaceTradersClient<Anonymous> {
    /// Return a paginated list of all the factions in the game.
    pub async fn list_factions(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Faction>, Meta), crate::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let request = self.get("factions")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataAndMetaWrapper::<Faction>>(json)?.inner())
    }

    /// View the details of a faction.
    pub async fn get_faction(&self, faction_symbol: &str) -> Result<Faction, crate::Error> {
        let request = self.get(&format!("factions/{faction_symbol}"))
            .send()
            .await?;
        let json = request
            .json::<serde_json::Value>()
            .await?;
        Ok(deserialize::<DataWrapper::<Faction>>(json)?.inner())
    }
}
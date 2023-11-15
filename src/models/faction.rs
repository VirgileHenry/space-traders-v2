use serde::Deserialize;

use crate::{
    client::{
        Authenticated,
        Anonymous
    },
    utils::{
        wrapper::{
            DataAndMetaWrapper,
            DataWrapper
        },
        pagination::page_limit_and_index
    }, error::server_error::ServerError
};

use super::meta::Meta;


/// All existing factions.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionType {
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
    symbol: FactionType,
    name: String,
    description: String,
    headquarters: String,
    traits: Vec<FactionTrait>,
    is_recruiting: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FactionSymbol {
    symbol: FactionType,
}

impl crate::SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all the factions in the game.
    pub async fn list_factions(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Faction>, Meta), crate::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("factions")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataAndMetaWrapper::<Faction>>::deserialize(json)?.inner())
            }
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// View the details of a faction.
    pub async fn get_faction(&self, faction_symbol: &str) -> Result<Faction, crate::Error> {
        let response = self.get(&format!("factions/{faction_symbol}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Faction>>::deserialize(json)?.inner())
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

impl crate::SpaceTradersClient<Anonymous> {
    /// Return a paginated list of all the factions in the game.
    pub async fn list_factions(&self, page_limit: Option<usize>, page_index: Option<usize>) -> Result<(Vec<Faction>, Meta), crate::Error> {
        let (limit, page) = page_limit_and_index(page_limit, page_index);
        let response = self.get("factions")
            .query(&[("limit", limit), ("page", page)])
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataAndMetaWrapper::<Faction>>::deserialize(json)?.inner())
            }
            other => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Err(crate::Error::FromServerError(<ServerError>::deserialize(json)?))
            }
        }
    }

    /// View the details of a faction.
    pub async fn get_faction(&self, faction_symbol: &str) -> Result<Faction, crate::Error> {
        let response = self.get(&format!("factions/{faction_symbol}"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<Faction>>::deserialize(json)?.inner())
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
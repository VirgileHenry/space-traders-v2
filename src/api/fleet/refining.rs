use serde::{Serialize, Deserialize};
use crate::{
    client::Authenticated,
    utils::wrapper::DataWrapper,
    error::server_error::SpaceTraderError,
    schemas::{
        ship::ship_cargo::ShipCargo,
        cooldown::Cooldown,
        extraction::extraction_yield::ExtractionYield,
        trade_symbol::TradeSymbol
    },
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefiningResult {
    pub cargo: ShipCargo,
    pub cooldown: Cooldown,
    pub produced: Vec<ExtractionYield>,
    pub consumed: Vec<ExtractionYield>,
}

// TODO: we use ExtractionYield because it's what we will receive, but better name ?
// also ExtractionYield is the same as SiphonYield ?

/// Subset of the trade symbol that can be produced from ship refinig.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FromRefinedTradeSymbol {
    Iron,
    Copper,
    Silver,
    Gold,
    Aluminum,
    Platinum,
    Uranite,
    Meritium,
    Fuel,
}

impl Into<TradeSymbol> for FromRefinedTradeSymbol {
    fn into(self) -> TradeSymbol {
        match self {
            FromRefinedTradeSymbol::Iron => TradeSymbol::Iron,
            FromRefinedTradeSymbol::Copper => TradeSymbol::Copper,
            FromRefinedTradeSymbol::Silver => TradeSymbol::Silver,
            FromRefinedTradeSymbol::Gold => TradeSymbol::Gold,
            FromRefinedTradeSymbol::Aluminum => TradeSymbol::Aluminum,
            FromRefinedTradeSymbol::Platinum => TradeSymbol::Platinum,
            FromRefinedTradeSymbol::Uranite => TradeSymbol::Uranite,
            FromRefinedTradeSymbol::Meritium => TradeSymbol::Meritium,
            FromRefinedTradeSymbol::Fuel => TradeSymbol::Fuel,
        }
    }
}

pub struct TradeSymbolCanBeFromRefiner;
impl TryFrom<TradeSymbol> for FromRefinedTradeSymbol {
    type Error = TradeSymbolCanBeFromRefiner;
    fn try_from(value: TradeSymbol) -> Result<Self, Self::Error> {
        match value {
            TradeSymbol::Iron => Ok(FromRefinedTradeSymbol::Iron),
            TradeSymbol::Copper => Ok(FromRefinedTradeSymbol::Copper),
            TradeSymbol::Silver => Ok(FromRefinedTradeSymbol::Silver),
            TradeSymbol::Gold => Ok(FromRefinedTradeSymbol::Gold),
            TradeSymbol::Aluminum => Ok(FromRefinedTradeSymbol::Aluminum),
            TradeSymbol::Platinum => Ok(FromRefinedTradeSymbol::Platinum),
            TradeSymbol::Uranite => Ok(FromRefinedTradeSymbol::Uranite),
            TradeSymbol::Meritium => Ok(FromRefinedTradeSymbol::Meritium),
            TradeSymbol::Fuel => Ok(FromRefinedTradeSymbol::Fuel),
            _ => Err(TradeSymbolCanBeFromRefiner),
        }
    }
}

impl crate::client::SpaceTradersClient<Authenticated> {
    /// Attempt to refine the raw materials on your ship. The request will only succeed if your ship is capable of refining at the time of the request. In order to be able to refine, a ship must have goods that can be refined and have installed a Refinery module that can refine it.
    ///
    /// When refining, 30 basic goods will be converted into 10 processed goods.
    pub async fn ship_refine(&self, ship_symbol: &str, refine_into: FromRefinedTradeSymbol) -> Result<RefiningResult, crate::error::Error> {
        let body = serde_json::json!({
            "produce": refine_into,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/refine"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<RefiningResult>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <SpaceTraderError>::deserialize(json)?; 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }
}
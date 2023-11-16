use std::num::NonZeroU64;

use serde::Deserialize;

/// The type of trade good (export, import, or exchange).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketTradeGoodType {
    Export,
    Import,
    Exchange,
}

/// The supply level of a trade good.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketTradeGoodSupply {
    Scarce,
    Limited,
    Moderate,
    High,
    Abundant,
}

/// The activity level of a trade good. If the good is an import, this represents how strong consumption is for the good. If the good is an export, this represents how strong the production is for the good.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketTradeGoodActivity {
    Weak,
    Growing,
    Strong,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketTradeGood {
    /// The symbol of the trade good.
    pub symbol: String,
    /// The type of trade good (export, import, or exchange).
    #[serde(rename = "type")]
    pub trade_type: MarketTradeGoodType,
    /// This is the maximum number of units that can be purchased or sold at this market in a single trade for this good. Trade volume also gives an indication of price volatility. A market with a low trade volume will have large price swings, while high trade volume will be more resilient to price changes.
    pub trade_volume: NonZeroU64,
    /// The supply level of a trade good.
    pub supply: MarketTradeGoodSupply,
    /// The activity level of a trade good. If the good is an import, this represents how strong consumption is for the good. If the good is an export, this represents how strong the production is for the good.
    pub activity: MarketTradeGoodActivity,
    /// The price at which this good can be purchased from the market.
    pub purchase_price: u64,
    /// The price at which this good can be sold to the market.
    pub sell_price: u64,
}
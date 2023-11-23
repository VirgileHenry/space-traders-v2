
use std::num::{NonZeroU64, NonZeroU32};

use serde::Deserialize;
use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::wrapper::{DataWrapper, ErrorWrapper},
    error::server_error::SpaceTraderError,
    schemas::{
        ship::ship_cargo::ShipCargo,
        trade_symbol::TradeSymbol,
        agent::Agent,
        market::market_transaction::MarketTransaction
    },
};


/// Result struct when making a cargo transaction.
/// Wrapper around an agent, cargo and transaction.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CagoTransactionResult {
    pub agent: Agent,
    pub cargo: ShipCargo,
    pub transaction: MarketTransaction,
}

impl SpaceTradersClient<Authenticated> {
    /// Retrieve the cargo of a ship under your agent's ownership.
    pub async fn get_ship_cargo(&self, ship_symbol: &str) -> Result<ShipCargo, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/cargo"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipCargo>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ErrorWrapper<SpaceTraderError>>::deserialize(json)?.inner(); 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Jettison cargo from your ship's cargo hold.
    pub async fn jettison_cargo(&self, ship_symbol: &str, to_jettison_symbol: TradeSymbol, jettison_amount: NonZeroU32) -> Result<ShipCargo, crate::error::Error> {
        let body = serde_json::json!({
            "symbol": to_jettison_symbol,
            "units": jettison_amount,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/jettison"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipCargo>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ErrorWrapper<SpaceTraderError>>::deserialize(json)?.inner(); 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Sell cargo in your ship to a market that trades this cargo. The ship must be docked in a waypoint that has the Marketplace trait in order to use this function.
    pub async fn sell_cargo(&self, ship_symbol: &str, to_sell_symbol: TradeSymbol, sell_amount: NonZeroU64) -> Result<CagoTransactionResult, crate::error::Error> {
        let body = serde_json::json!({
            "symbol": to_sell_symbol,
            "units": sell_amount,
        });
        let response = self.post(&format!("my/ships/{ship_symbol}/sell"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<CagoTransactionResult>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ErrorWrapper<SpaceTraderError>>::deserialize(json)?.inner(); 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Purchase cargo from a market.
    /// 
    /// The ship must be docked in a waypoint that has Marketplace trait, and the market must be selling a good to be able to purchase it.
    /// 
    /// The maximum amount of units of a good that can be purchased in each transaction are denoted by the tradeVolume value of the good, which can be viewed by using the Get Market action.
    /// 
    /// Purchased goods are added to the ship's cargo hold.
    pub async fn purchase_cargo(&self, ship_symbol: &str, to_purchase_symbol: TradeSymbol, purchase_amount: NonZeroU64) -> Result<CagoTransactionResult, crate::error::Error> {
        let body = serde_json::json!({
            "symbol": to_purchase_symbol,
            "units": purchase_amount,
        });
        let response = self.get(&format!("my/ships/{ship_symbol}/purchase"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<CagoTransactionResult>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ErrorWrapper<SpaceTraderError>>::deserialize(json)?.inner(); 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Transfer cargo between ships.
    /// 
    /// The receiving ship must be in the same waypoint as the transferring ship, and it must able to hold the additional cargo after the transfer is complete. Both ships also must be in the same state, either both are docked or both are orbiting.
    /// 
    /// The response body's cargo shows the cargo of the transferring ship after the transfer is complete.
    pub async fn transfer_cargo(&self, from_ship_symbol: &str, to_transfer_symbol: TradeSymbol, transfer_amount: NonZeroU64, to_ship_symbol: &str) -> Result<ShipCargo, crate::error::Error> {
        let body = serde_json::json!({
            "symbol": to_transfer_symbol,
            "units": transfer_amount,
            "shipSymbol": to_ship_symbol,
        });
        let response = self.get(&format!("my/ships/{from_ship_symbol}/transfer"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ShipCargo>>::deserialize(json)?.inner())
            }
            status => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                let server_error = <ErrorWrapper<SpaceTraderError>>::deserialize(json)?.inner(); 
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

}
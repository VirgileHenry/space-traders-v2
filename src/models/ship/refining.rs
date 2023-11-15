use serde::{Serialize, Deserialize};

use crate::{
    client::Authenticated,
    utils::wrapper::DataWrapper,
    models::{
        cargo::Cargo,
        cooldown::Cooldown
    },
    error::server_error::ServerError
};


#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefinedGoodType {
    Iron,
    Copper,
    Silver,
    Gold,
    Aluminium,
    Platinium,
    Uranite,
    Meritium,
    Fuel,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefiningResult {
    cargo: Cargo,
    cooldown: Cooldown,
    produced: Vec<Goods>,
    consumed: Vec<Goods>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Goods {
    trade_symbol: String,
    units: i64,
}



impl crate::client::SpaceTradersClient<Authenticated> {
    /// Attempt to refine the raw materials on your ship. The request will only succeed if your ship is capable of refining at the time of the request. In order to be able to refine, a ship must have goods that can be refined and have installed a Refinery module that can refine it.
    ///
    /// When refining, 30 basic goods will be converted into 10 processed goods.
    pub async fn ship_refine(&self, ship_symbol: &str, refine_into: RefinedGoodType) -> Result<RefiningResult, crate::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/refine"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<RefiningResult>>::deserialize(json)?.inner())
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
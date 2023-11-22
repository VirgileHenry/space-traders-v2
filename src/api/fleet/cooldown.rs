use serde::Deserialize;
use crate::{
    client::Authenticated,
    utils::wrapper::DataWrapper,
    error::server_error::SpaceTraderError,
    schemas::cooldown::Cooldown
};


impl crate::client::SpaceTradersClient<Authenticated> {
    /// Retrieve the details of your ship's reactor cooldown. Some actions such as activating your jump drive, scanning, or extracting resources taxes your reactor and results in a cooldown.
    /// 
    /// Your ship cannot perform additional actions until your cooldown has expired. The duration of your cooldown is relative to the power consumption of the related modules or mounts for the action taken.
    /// 
    /// Response returns a 204 status code (no-content) when the ship has no cooldown.
    pub async fn get_ship_cooldown(&self, ship_symbol: &str) -> Result<Option<Cooldown>, crate::error::Error> {
        let response = self.get(&format!("my/ships/{ship_symbol}/cooldown"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(Some(<DataWrapper::<Cooldown>>::deserialize(json)?.inner()))
            }
            204 => Ok(None), // no cooldown
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



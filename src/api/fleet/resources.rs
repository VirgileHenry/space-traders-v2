use serde::Deserialize;
use crate::{
    client::{Authenticated, SpaceTradersClient},
    utils::wrapper::DataWrapper,
    error::server_error::SpaceTraderError,
    schemas::{
        ship::ship_cargo::ShipCargo,
        survey::Survey,
        cooldown::Cooldown,
        extraction::extraction_yield::ExtractionYield, siphon::siphon_yield::SiphonYield
    },
};

/// Wrapper around a cooldown, extraction yield and cargo.
/// This is returned by the extract_resource method.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtractResourcesResponse {
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub cooldown: Cooldown,
    /// Extraction details.
    pub extraction: ExtractionYield,
    /// Ship cargo details.
    pub cargo: ShipCargo,
}

/// Wrapper around a cooldown, extraction yield and cargo.
/// This is returned by the extract_resource method.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SiphonResourcesResponse {
    /// A cooldown is a period of time in which a ship cannot perform certain actions.
    pub cooldown: Cooldown,
    /// Siphon details.
    pub siphon: SiphonYield,
    /// Ship cargo details.
    pub cargo: ShipCargo,
}

impl SpaceTradersClient<Authenticated> {
    /// Extract resources from a waypoint that can be extracted, such as asteroid fields, into your ship. Send an optional survey as the payload to target specific yields.
    /// 
    /// The ship must be in orbit to be able to extract and must have mining equipments installed that can extract goods, such as the Gas Siphon mount for gas-based goods or Mining Laser mount for ore-based goods.
    /// 
    /// The survey property is now deprecated. See the extract/survey endpoint for more details.
    pub async fn exract_resources(&self, ship_symbol: &str, #[warn(deprecated)] survey: Option<&Survey>) -> Result<ExtractResourcesResponse, crate::error::Error> {
        let body = match survey {
            Some(survey) => serde_json::json!({
                "survey": survey,
            }),
            None => serde_json::json!({}),
        };
        let response = self.post(&format!("my/ships/{ship_symbol}/extract"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ExtractResourcesResponse>>::deserialize(json)?.inner())
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

    /// Siphon gases, such as hydrocarbon, from gas giants.
    /// 
    /// The ship must be in orbit to be able to siphon and must have siphon mounts and a gas processor installed.
    pub async fn siphon_resources(&self, ship_symbol: &str) -> Result<SiphonResourcesResponse, crate::error::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/siphon"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<SiphonResourcesResponse>>::deserialize(json)?.inner())
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

    /// Use a survey when extracting resources from a waypoint. This endpoint requires a survey as the payload, which allows your ship to extract specific yields.
    /// 
    /// Send the full survey object as the payload which will be validated according to the signature. If the signature is invalid, or any properties of the survey are changed, the request will fail.
    pub async fn exract_resources_with_survey(&self, ship_symbol: &str, survey: &Survey) -> Result<ExtractResourcesResponse, crate::error::Error> {
        let body = serde_json::to_value(survey)?;
        let response = self.post(&format!("my/ships/{ship_symbol}/extract/survey"))
            .json(&body)
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<ExtractResourcesResponse>>::deserialize(json)?.inner())
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
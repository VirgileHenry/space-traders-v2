use serde::Deserialize;
use crate::{
    client::Authenticated,
    utils::wrapper::{DataWrapper, ErrorWrapper},
    error::server_error::SpaceTraderError,
    schemas::{cooldown::Cooldown, survey::Survey},
};

/// Wrapper around a cooldown and surveys.
/// Result of a create_survey request.  
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CooldownAndSurveys {
    pub cooldown: Cooldown,
    pub surveys: Vec<Survey>,
}

impl crate::client::SpaceTradersClient<Authenticated> {
    /// Create surveys on a waypoint that can be extracted such as asteroid fields. A survey focuses on specific types of deposits from the extracted location. When ships extract using this survey, they are guaranteed to procure a high amount of one of the goods in the survey.
    /// 
    /// In order to use a survey, send the entire survey details in the body of the extract request.
    /// 
    /// Each survey may have multiple deposits, and if a symbol shows up more than once, that indicates a higher chance of extracting that resource.
    /// 
    /// Your ship will enter a cooldown after surveying in which it is unable to perform certain actions. Surveys will eventually expire after a period of time or will be exhausted after being extracted several times based on the survey's size. Multiple ships can use the same survey for extraction.
    /// 
    /// A ship must have the Surveyor mount installed in order to use this function.
    pub async fn create_survey(&self, ship_symbol: &str) -> Result<CooldownAndSurveys, crate::error::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/survey"))
            .send()
            .await?;
        match response.status().as_u16() {
            201 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<CooldownAndSurveys>>::deserialize(json)?.inner())
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
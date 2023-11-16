use serde::Deserialize;
use crate::{client::Authenticated, utils::wrapper::DataWrapper, error::server_error::ServerError, models::{cooldown::Cooldown, survey::Survey}};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CooldownAndSurveys {
    cooldown: Cooldown,
    surveys: Vec<Survey>,
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
    pub async fn create_survey(&self, ship_symbol: &str) -> Result<CooldownAndSurveys, crate::Error> {
        let response = self.post(&format!("my/ships/{ship_symbol}/srvey"))
            .send()
            .await?;
        match response.status().as_u16() {
            200 => {
                let json = response
                    .json::<serde_json::Value>()
                    .await?;
                Ok(<DataWrapper::<CooldownAndSurveys>>::deserialize(json)?.inner())
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
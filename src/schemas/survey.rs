pub mod survey_deposit;

use serde::{Serialize, Deserialize};

use self::survey_deposit::SurveyDeposit;

/// The size of the deposit. This value indicates how much can be extracted from the survey before it is exhausted.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SurveySize {
    Small,
    Moderate,
    Large,
}

/// A resource survey of a waypoint, detailing a specific extraction location and the types of resources that can be found there.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    /// A unique signature for the location of this survey. This signature is verified when attempting an extraction using this survey.
    pub signature: String,
    /// The symbol of the waypoint that this survey is for.
    pub symbol: String,
    /// A list of deposits that can be found at this location. A ship will extract one of these deposits when using this survey in an extraction request. If multiple deposits of the same type are present, the chance of extracting that deposit is increased.
    pub deposits: Vec<SurveyDeposit>,
    /// The date and time when the survey expires. After this date and time, the survey will no longer be available for extraction.
    pub expiration: chrono::DateTime<chrono::Utc>,
    /// The size of the deposit. This value indicates how much can be extracted from the survey before it is exhausted.
    pub size: SurveySize,
}

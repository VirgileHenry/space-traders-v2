use std::num::NonZeroU64;

use serde::Deserialize;

use crate::models::{ship::requirements::Requirements, resource::Resource};

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReactorSymbol {
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
}

pub struct NotAReactorError;
impl TryFrom<Resource> for ReactorSymbol {
    type Error = NotAReactorError;
    fn try_from(value: Resource) -> Result<Self, Self::Error> {
        match value {
            Resource::ReactorSolarI => Ok(ReactorSymbol::ReactorSolarI),
            Resource::ReactorFusionI => Ok(ReactorSymbol::ReactorFusionI),
            Resource::ReactorFissionI => Ok(ReactorSymbol::ReactorFissionI),
            Resource::ReactorChemicalI => Ok(ReactorSymbol::ReactorChemicalI),
            Resource::ReactorAntimatterI => Ok(ReactorSymbol::ReactorAntimatterI),
            _ => Err(NotAReactorError)
        }
    }
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
    symbol: ReactorSymbol,
    name: String,
    description: String,
    condition: Option<u64>,
    power_output: NonZeroU64,
    requirements: Requirements,
}



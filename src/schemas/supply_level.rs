use serde::Deserialize;

/// The supply level of a trade good.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SupplyLevel {
    Scarce,
    Limited,
    Moderate,
    High,
    Abundant,
}
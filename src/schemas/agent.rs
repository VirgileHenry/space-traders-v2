use serde::Deserialize;

/// Agent details.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    /// Account ID that is tied to this agent. Only included on your own agent.
    pub account_id: Option<String>,
    /// Symbol of the agent.
    pub symbol: String,
    /// The headquarters of the agent.
    pub headquarters: String,
    /// The number of credits the agent has available. Credits can be negative if funds have been overdrawn.
    pub credits: i64,
    /// The faction the agent started with.
    pub starting_faction: String,
    /// How many ships are owned by the agent.
    pub ship_count: u64,
}

impl PartialEq for Agent {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl Eq for Agent {}

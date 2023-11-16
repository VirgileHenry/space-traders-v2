use std::num::NonZeroU64;
use serde::Deserialize;

/// Meta details for pagination.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// Shows the total amount of items of this kind that exist.
    pub total: u64,
    /// A page denotes an amount of items, offset from the first item. Each page holds an amount of items equal to the limit.
    pub page: NonZeroU64,
    /// The amount of items in each page. Limits how many items can be fetched at once.
    pub limit: NonZeroU64,
}
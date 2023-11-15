use serde::Deserialize;


/// General meta data about requests.
/// This is received with paginated requests,
/// containing information about pagination.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    total: u64,
    page: u64,
    limit: u64,
}
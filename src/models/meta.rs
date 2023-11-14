use serde::Deserialize;


/// General meta data about requests.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    total: u64,
    page: u64,
    limit: u64,
}

/// Convenient function to default pagination limit and index to default if none are provided.
/// The provided values are clamped in the correct range.
pub(crate) fn page_limit_and_index(page_limit: Option<u64>, page_index: Option<u64>) -> (u64, u64) {
    (
        match page_limit {
            Some(limit) => limit.min(20).max(1),
            None => 10,
        },
        match page_index {
            Some(index) => index.min(1),
            None => 1,
        },
    )
}
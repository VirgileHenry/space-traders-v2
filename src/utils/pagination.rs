
/// Convenient function to default pagination limit and index to default if none are provided.
pub(crate) fn page_limit_and_index(page_limit: Option<usize>, page_index: Option<usize>) -> (usize, usize) {
    (
        match page_limit {
            Some(limit) => limit,
            None => 10,
        },
        match page_index {
            Some(index) => index,
            None => 1,
        },
    )
}
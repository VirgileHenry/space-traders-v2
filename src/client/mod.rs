
// the idea was cool and all, but the test url is spitting out nonsense
// #[cfg(any(test, debug_assertions))]
// const SPACE_TRADERS_API: &'static str  = "https://stoplight.io/mocks/spacetraders/spacetraders/96627693";
// #[cfg(all(not(test), not(debug_assertions)))]
const SPACE_TRADERS_API: &'static str  = "https://api.spacetraders.io/v2";

pub trait AuthState {}

pub struct Authenticated(String);
pub struct Anonymous;

impl AuthState for Authenticated {}
impl AuthState for Anonymous {}

/// A client to play the game.
pub struct SpaceTradersClient<A: AuthState> {
    http_client: reqwest::Client,
    auth_token: A,
}

impl SpaceTradersClient<Anonymous> {
    pub fn new_anonymous() -> SpaceTradersClient<Anonymous> {
        SpaceTradersClient {
            http_client: reqwest::Client::new(),
            auth_token: Anonymous,
        }
    }

    pub fn auth(self, token: &str) -> SpaceTradersClient<Authenticated> {
        SpaceTradersClient {
            http_client: self.http_client,
            auth_token: Authenticated(token.to_string())
        }
    }

    pub(crate) fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.http_client.get(format!("{}/{}", SPACE_TRADERS_API, path))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
    }

    pub(crate) fn post(&self, path: &str) -> reqwest::RequestBuilder {
        self.http_client.post(format!("{}/{}", SPACE_TRADERS_API, path))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
    }
}

impl SpaceTradersClient<Authenticated> {
    pub fn new_with_auth(auth_token: &str) -> SpaceTradersClient<Authenticated> {
        SpaceTradersClient {
            http_client: reqwest::Client::new(),
            auth_token: Authenticated(auth_token.to_string()),
        }
    }

    pub fn de_auth(self) -> SpaceTradersClient<Anonymous> {
        SpaceTradersClient {
            http_client: self.http_client,
            auth_token: Anonymous
        }
    }

    pub(crate) fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.http_client.get(format!("{}/{}", SPACE_TRADERS_API, path))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.auth_token.0))
    }

    pub(crate) fn post(&self, path: &str) -> reqwest::RequestBuilder {
        self.http_client.post(format!("{}/{}", SPACE_TRADERS_API, path))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.auth_token.0))
    }

    pub(crate) fn patch(&self, path: &str) -> reqwest::RequestBuilder {
        self.http_client.patch(format!("{}/{}", SPACE_TRADERS_API, path))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.auth_token.0))
    }
}
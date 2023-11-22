use serde::Deserialize;
use crate::{
    client::{
        SpaceTradersClient,
        Anonymous,
        Authenticated
    },
    schemas::{
        meta::Meta,
        waypoint::Waypoint,
        market::Market,
        shipyard::Shipyard,
        jump_gate::JumpGate,
        construction::Construction,
        trade_symbol::TradeSymbol,
        ship::ship_cargo::ShipCargo
    },
    utils::wrapper::{PaginationWrapper, DataWrapper},
    error::server_error::SpaceTraderError
};

impl SpaceTradersClient<Anonymous> {
    /// Return a paginated list of all of the waypoints for a given system.
    /// 
    /// If a waypoint is uncharted, it will return the Uncharted trait instead of its actual traits.
    pub async fn list_waypoints_in_system(&self, system_symbol: &str) -> Result<(Vec<Waypoint>, Meta), crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(PaginationWrapper::<Waypoint>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get the details of a system.
    pub async fn get_waypoint(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Waypoint, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Waypoint>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Retrieve imports, exports and exchange data from a marketplace. Requires a waypoint that has the Marketplace trait to use.
    /// 
    /// Send a ship to the waypoint to access trade good prices and recent transactions. Refer to the Market Overview page to gain better a understanding of the market in the game.
    pub async fn get_market(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Market, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/market")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Market>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get the shipyard for a waypoint. Requires a waypoint that has the Shipyard trait to use. Send a ship to the waypoint to access data on ships that are currently available for purchase and recent transactions.
    pub async fn get_shipyard(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Shipyard, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/shipyard")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Shipyard>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get jump gate details for a waypoint. Requires a waypoint of type JUMP_GATE to use.
    /// 
    /// Waypoints connected to this jump gate can be
    pub async fn get_jumpgate(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<JumpGate, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/jump-gate")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<JumpGate>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get construction details for a waypoint. Requires a waypoint with a property of isUnderConstruction to be true.
    pub async fn get_construction_site(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Construction, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/construction")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Construction>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }
}


/// Wrapper around a construction and a cargo.
/// This is returned when supplying materials to a jump gate.
#[derive(Deserialize, Clone, Debug)]
pub struct ConstructionAndCargo {
    /// The construction details of a waypoint.
    pub construction: Construction,
    /// Ship cargo details.
    pub cargo: ShipCargo,
}


impl SpaceTradersClient<Authenticated> {
    /// Return a paginated list of all of the waypoints for a given system.
    /// 
    /// If a waypoint is uncharted, it will return the Uncharted trait instead of its actual traits.
    pub async fn list_waypoints_in_system(&self, system_symbol: &str) -> Result<(Vec<Waypoint>, Meta), crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(PaginationWrapper::<Waypoint>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get the details of a system.
    pub async fn get_waypoint(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Waypoint, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Waypoint>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Retrieve imports, exports and exchange data from a marketplace. Requires a waypoint that has the Marketplace trait to use.
    /// 
    /// Send a ship to the waypoint to access trade good prices and recent transactions. Refer to the Market Overview page to gain better a understanding of the market in the game.
    pub async fn get_market(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Market, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/market")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Market>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }


    /// Get the shipyard for a waypoint. Requires a waypoint that has the Shipyard trait to use. Send a ship to the waypoint to access data on ships that are currently available for purchase and recent transactions.
    pub async fn get_shipyard(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Shipyard, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/shipyard")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Shipyard>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get jump gate details for a waypoint. Requires a waypoint of type JUMP_GATE to use.
    /// 
    /// Waypoints connected to this jump gate can be
    pub async fn get_jumpgate(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<JumpGate, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/jump-gate")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<JumpGate>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Get construction details for a waypoint. Requires a waypoint with a property of isUnderConstruction to be true.
    pub async fn get_construction_site(&self, system_symbol: &str, waypoint_symbol: &str) -> Result<Construction, crate::error::Error> {
        let response = self.get(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/construction")).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<Construction>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

    /// Supply a construction site with the specified good. Requires a waypoint with a property of isUnderConstruction to be true.
    /// 
    /// The good must be in your ship's cargo. The good will be removed from your ship's cargo and added to the construction site's materials.
    pub async fn supply_construction_site(&self, system_symbol: &str, waypoint_symbol: &str, ship_symbol: &str, trade_symbol: TradeSymbol, amount: i64) -> Result<ConstructionAndCargo, crate::error::Error> {
        let body = serde_json::json!({
            "shipSymbol": ship_symbol,
            "tradeSymbol": trade_symbol,
            "units": amount,
        });
        let response = self.post(&format!("systems/{system_symbol}/waypoints/{waypoint_symbol}/construction/supply")).json(&body).send().await?;
        match response.status().as_u16() {
            200 => {
                let json = response.json::<serde_json::Value>().await?;
                Ok(DataWrapper::<ConstructionAndCargo>::deserialize(json)?.inner())
            },
            status => {
                let json = response.json::<serde_json::Value>().await?;
                let server_error = <SpaceTraderError>::deserialize(json)?;
                Err(crate::error::Error::from((status, server_error)))
            }
        }
    }

}
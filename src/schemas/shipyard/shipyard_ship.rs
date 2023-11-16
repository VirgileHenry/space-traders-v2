use serde::Deserialize;

use crate::schemas::{
    ship::{
        ship_type::ShipType,
        ship_frame::ShipFrame,
        ship_reactor::ShipReactor,
        ship_engine::ShipEngine,
        ship_module::ShipModule,
        ship_mount::ShipMount,
        ship_crew::ShipCrew
    },
    market::market_trade_good::MarketTradeGoodActivity,
    supply_level::SupplyLevel,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShip {
    /// Type of ship
    #[serde(rename = "type")]
    pub ship_type: ShipType,
    /// The name of the ship
    pub name: String,
    /// The description of the ship
    pub description: String,
    /// The supply level of a trade good.
    pub supply: SupplyLevel,
    /// The activity level of a trade good. If the good is an import, this represents how strong consumption is for the good. If the good is an export, this represents how strong the production is for the good.
    pub activity: MarketTradeGoodActivity,
    /// 
    pub purchase_price: i64,
    /// The frame of the ship. The frame determines the number of modules and mounting points of the ship, as well as base fuel capacity. As the condition of the frame takes more wear, the ship will become more sluggish and less maneuverable.
    pub frame: ShipFrame,
    /// The reactor of the ship. The reactor is responsible for powering the ship's systems and weapons.
    pub reactor: ShipReactor,
    /// The engine determines how quickly a ship travels between waypoints.
    pub engine: ShipEngine,
    /// A module can be installed in a ship and provides a set of capabilities such as storage space or quarters for crew. Module installations are permanent.
    pub modules: Vec<ShipModule>,
    /// A mount is installed on the exterier of a ship.
    pub mounts: Vec<ShipMount>,
    /// 
    pub crew: ShipCrew,
}
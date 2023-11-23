use crate::{client::SpaceTradersClient, schemas::{waypoint::{waypoint_trait::WaypointTraitType, waypoint_type::WaypointType}, ship::{ship_type::ShipType, ship_nav::ship_nav_flight_mode::ShipNavFlightMode}}};

#[tokio::test]
async fn tutorial() {
    // create a client to cache http endpoint
    let client = SpaceTradersClient::new_anonymous();
    // get server status
    let server_status = client.get_server_status().await.unwrap();
    println!("\nPlaying tutorial on server with status:\n{server_status:?}");
    // create a new bot
    let agent = client.create_agent(
        "your.email@example.com",
        "COSMIC",
        "BIP-BOP-BOT-47"
    ).await.unwrap();
    let client = client.auth(&agent.token);
    println!("\nNow playing as agent {agent:?}");
    // get our current location.
    println!("\nCurrent waypoint : {}", agent.ship.nav.waypoint_symbol);
    // accept the contract
    let contract = client.accept_contract(&agent.contract.id).await.unwrap().contract;
    println!("\nAccepted contract: {:?}", contract);
    // look for a waypoint with shipyard trait, where we have one of our ships there and is an orbital station (tutorial shipyard)
    let required_traits = vec![WaypointTraitType::Shipyard];
    let (ships, _meta) = client.list_ships(None, None).await.unwrap();
    let waypoint_with_ships = ships.into_iter().map(|ship| ship.nav.waypoint_symbol).collect::<Vec<_>>();
    let (mut waypoints, mut meta) = client.list_waypoints_in_system(&agent.ship.nav.system_symbol, None, None, Some(&required_traits), None).await.unwrap();
    let shipyard_waypoint = loop {
        let orbital_stations_waypoint = waypoints.into_iter()
            .filter(|w| waypoint_with_ships.iter().any(|c_w| c_w == &w.symbol))
            .collect::<Vec<_>>();
        if !orbital_stations_waypoint.is_empty() {
            break orbital_stations_waypoint.into_iter().next().unwrap();
        }
        if meta.total <= meta.limit.saturating_mul(meta.page).into() {
            // end of waypoints, nothing matched, stop cond
            println!("\nNo waypoints with shipyard in system. Can't continue !");
            return;
        }
        (waypoints, meta) = client.list_waypoints_in_system(&agent.ship.nav.system_symbol, None, Some(meta.page.saturating_add(1).into()), Some(&required_traits), None).await.unwrap();
    };
    // look for the shipyard 
    let shipyard = client.get_shipyard(&shipyard_waypoint.system_symbol, &shipyard_waypoint.symbol).await.unwrap();
    println!("\nFound a shipyard {:?} at waypoint {:?}", shipyard, shipyard_waypoint);
    // buy a mining drone ship
    let buy_ship_response = client.purchase_ship(ShipType::ShipMiningDrone, &shipyard_waypoint.symbol).await.unwrap();
    println!("\nBought a mining ship: {:?}", buy_ship_response.transaction);
    // loof for an asteroid to mine
    let (waypoints, _meta) = client.list_waypoints_in_system(&agent.ship.nav.system_symbol, None, None, None, Some(WaypointType::EngineeredAsteroid)).await.unwrap();
    if waypoints.is_empty() {
        println!("\nNo engineered asteroid waypoiny. Can't continue !");
        return;
    }
    let to_mine_asteroid = waypoints.into_iter().next().unwrap();
    println!("\nFound engineered asteroid waypoint: {to_mine_asteroid:?}");
    // orbit the drone
    let drone = buy_ship_response.ship;
    let new_drone_nav = client.orbit_ship(&drone.symbol).await.unwrap();
    println!("\nDrone set to orbit: {:?}", new_drone_nav);
    // navigate to the engineered asteroid
    let nav_result = client.navigate_ship(&drone.symbol, &to_mine_asteroid.symbol).await.unwrap();
    println!("\nDrone navigating to asteroid: {:?}", nav_result);
    // burst to nav quicker
    let patch_result = client.patch_ship_nav(&drone.symbol, ShipNavFlightMode::Burn).await.unwrap();
    println!("\nDrone bursted to get to asteroid quicker: {:?}", patch_result);
    // wait until the drone arrived
    let arrival_time = patch_result.route.arrival;
    println!("\nDrone will arrive at {}", arrival_time);
    let now = chrono::offset::Utc::now();
    let waiting_time = arrival_time - now;
    match waiting_time.to_std() {
        Ok(positive_duration) => {
            println!("\nSleeping for {positive_duration:?}");
            tokio::time::sleep(positive_duration).await;
        },
        Err(_) => {},
    }
    println!("\nDrone arrived!");
    // dock the ship to refuel
    client.dock_ship(&drone.symbol).await.unwrap();
    let refuel_info = client.refuel_ship(&drone.symbol, None).await.unwrap();
    println!("\nDrone refueled: {:?}", refuel_info.fuel);
    // orbit the drone to mine
    client.orbit_ship(&drone.symbol).await.unwrap();
    // mine asteroid
    let extraction = client.exract_resources(&drone.symbol, None).await.unwrap();
    println!("Drone extracted resources: {extraction:?}");
    
}

use crate::client::SpaceTradersClient;

#[tokio::test]
async fn tutorial() {
    // create a client to cache http endpoint
    let client = SpaceTradersClient::new_anonymous();
    // get server status
    let server_status = client.get_server_status().await.unwrap();
    println!("Playing tutorial on server with status:\n{server_status:?}");
    // create a new bot
    let agent = client.create_agent("your.email@example.com", "COSMIC", "BIP-BOP-BOT").await.unwrap();
    let client = client.auth(&agent.token);
    println!("Now playing as agent {agent:?}");
    // lists all our ships
    let (ships, _meta) = client.list_ships(None, None).await.unwrap();
    println!("Ships we own: {ships:?}");

}
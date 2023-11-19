use crate::client::SpaceTradersClient;

#[tokio::test]
async fn tutorial() {
    // create a client to cache http endpoint
    let client = SpaceTradersClient::new_anonymous();
    // get a list of all systems (paginated).
    let (systems, pagination) = client.list_systems().await.unwrap();
    println!("systems (paginated: {pagination:?}:\n");
    for system in systems {
        println!("{system:?}\n");
    }

}
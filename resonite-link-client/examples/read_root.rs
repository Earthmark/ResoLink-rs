#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("The first argument must be the url to connect to.");
        return;
    }

    let address = args[1].clone();

    let client = resonite_link_client::Client::connect(&address, None)
        .await
        .unwrap();

    let response = client
        .send(resonite_link_client::Message::GetSlot {
            slot_id: "Root".into(),
            depth: 3,
            include_component_data: true,
        })
        .await
        .unwrap();

    println!("{:#?}", response);
}

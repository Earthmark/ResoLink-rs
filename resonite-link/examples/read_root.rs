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
        .send(resonite_link_client::Message::new_get_slot("Root", 0, false))
        .await
        .unwrap();

    println!("{:#?}", response);
}

mod minecraft;
mod utils;

use minecraft::application::Client;

// use utils::logging::get_logger;


fn main() {
    // initialize the client
    let mut client = Client::new("127.0.0.1", 55916, "Entity");
    client.connect();
    /*
    let statusresponse = client.status_request();
    get_logger().info(format!("Status Response:\n{}{}", " ".repeat(53), statusresponse.get_players().get_players_count()));
    get_logger().info(format!("{}", statusresponse.get_description_text()));
    get_logger().info(format!("players:\n{}", statusresponse.get_players().get_players_list().join("\n")));
    match statusresponse.get_favicon() {
        None => {}
        Some(c) => {
            std::fs::write(
                "favicon.png",
                c.get_raw_image().as_ref().unwrap(),
            ).expect("IO Error");
        }
    }
    */
}

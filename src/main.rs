mod minecraft;
mod utils;

use minecraft::application::Client;

use crate::utils::logging::{get_logger, set_log_level};

fn main() {
    // set debug
    if cfg!(debug_assertions) {
        set_log_level(3);
    }

    // initialize the client
    let mut client = Client::new("localhost", 55916, "Entity1");

    // ping the server
    let statusresponse = client.status_request();
    get_logger().debug(format!("StatusResponse: {:?}", statusresponse));
    get_logger().info(format!(
        "Status Response:\n{}{}",
        " ".repeat(53),
        statusresponse.get_players().get_players_count()
    ));
    get_logger().info(format!("{}", statusresponse.get_description_text()));
    get_logger().info(format!(
        "players:\n{}",
        statusresponse.get_players().get_players_list().join("\n")
    ));
    if let Some(image) = statusresponse.get_favicon() {
        std::fs::write(
            "favicon.png",
            image.get_raw_image().expect("unable to load server icon"),
        )
        .expect("IO Error");
    }

    // join the server
    client.spawn_player();
}

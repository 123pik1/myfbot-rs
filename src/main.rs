use serenity::{
    Client,
    all::{GatewayIntents, Http},
};

use crate::simple_responder::SimpleResponder;

mod settings;
mod simple_responder;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::init();

    let http = Http::new(&settings.discord_bot_token);

    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&settings.discord_bot_token, intents)
        .event_handler(SimpleResponder)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

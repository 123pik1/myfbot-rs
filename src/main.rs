use serenity::{
    Client,
    all::{GatewayIntents, Http},
};

use crate::{command_responder::CommandResponder, simple_responder::SimpleResponder};

mod command_responder;
mod settings;
mod simple_responder;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::init();

    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&settings.discord_bot_token, intents)
        .event_handler(SimpleResponder)
        .event_handler(CommandResponder)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

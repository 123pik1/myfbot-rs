use serenity::{all::GatewayIntents, client::ClientBuilder};

use crate::{data::Data, simple_responder::SimpleResponder};

mod command_responder;
// Struct which will be accessible globally
mod data;
mod ollama;
mod settings;
mod simple_responder;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::init();
    let token = settings.discord_bot_token.clone();

    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![command_responder::age(), command_responder::ask()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { settings })
            })
        })
        .build();

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(SimpleResponder)
        .await
        .expect("Error starting client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

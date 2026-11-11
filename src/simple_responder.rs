use serenity::{
    all::{Context, EventHandler, Message, Ready},
    async_trait,
};

pub struct SimpleResponder;

#[async_trait]
impl EventHandler for SimpleResponder {
    async fn message(&self, ctx: Context, msg: Message) {
        #[allow(clippy::single_match)]
        match msg.content.as_str() {
            "!ping" => send_message(ctx, msg, "Pong!").await,
            "myf" => send_message(ctx, msg, "myf myf").await,
            _ => {}
        }
    }

    /// Ready informing function
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn send_message(ctx: Context, msg: Message, text: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
        println!("Error sending message: {why:?}")
    }
}

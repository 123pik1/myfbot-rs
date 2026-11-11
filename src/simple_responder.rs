use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};

pub struct SimpleResponder;

#[async_trait]
impl EventHandler for SimpleResponder {
    async fn message(&self, ctx: Context, msg: Message) {
        #[allow(clippy::single_match)]
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {why:?}")
                }
            }
            _ => {}
        }
    }
}

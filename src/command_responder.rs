use serenity::{
    all::{
        CacheHttp, Command, CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, Interaction, Ready,
    },
    async_trait,
};

pub struct CommandResponder;

#[async_trait]
impl EventHandler for CommandResponder {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Commands for {} are connected!", ready.user.name);

        // Register commands here
        let ping_command = CreateCommand::new("ping").description("Responses pong");
        if let Err(why) = Command::create_global_command(&ctx.http, ping_command).await {
            eprintln!("Błąd rejestracji komendy /ping: {why:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };

        #[allow(clippy::single_match)]
        match command.data.name.as_str() {
            "ping" => react_to_ping(ctx, command).await,
            _ => {}
        }
    }
}

async fn react_to_ping(ctx: Context, command: CommandInteraction) {
    let message = CreateInteractionResponseMessage::new().content("Pong!");
    let response = CreateInteractionResponse::Message(message);

    if let Err(why) = command.create_response(&ctx.http, response).await {
        eprintln!("Error with response to command: {why:?}");
    }
}

use crate::{
    data::{Context, Error},
    ollama::ask_ollama,
};
use serenity::all::User;

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "Prompt to local chatbot"] prompt: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let data = ctx.data();

    let response = ask_ollama(
        data.settings.model_url.clone(),
        data.settings.model_name.clone(),
        prompt,
        None,
    )
    .await;

    if let Some(res) = response {
        ctx.say(res.response).await?;
    } else {
        ctx.say("Error during calling ollama model").await?;
    }

    Ok(())
}

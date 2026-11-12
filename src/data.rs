use crate::settings::Settings;
use serenity::all::UserId;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub settings: Settings,
}

impl Data {
    pub fn is_me(&self, user_id: UserId) -> bool {
        user_id.get() == self.settings.discord_allowed_user_id
    }
}

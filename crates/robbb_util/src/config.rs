use std::{path::PathBuf, sync::Arc};

use poise::serenity_prelude::{ChannelId, CreateEmbed, GuildId, RoleId, TypeMapKey};
use serenity::client;

use crate::{
    extensions::GuildIdExt,
    log_error,
    util::{parse_required_env_var, required_env_var},
};

#[derive(Debug)]
pub struct Config {
    pub discord_token: String,

    pub guild: GuildId,
    pub role_mod: RoleId,
    pub role_helper: RoleId,
    pub role_mute: RoleId,
    pub roles_color: Vec<RoleId>,

    pub category_mod_private: ChannelId,
    pub channel_announcements: ChannelId,
    pub channel_rules: ChannelId,
    pub channel_showcase: ChannelId,
    pub channel_feedback: ChannelId,
    pub channel_modlog: ChannelId,
    pub channel_mod_bot_stuff: ChannelId,
    pub channel_auto_mod: ChannelId,
    pub channel_bot_messages: ChannelId,
    pub channel_bot_traffic: ChannelId,
    pub channel_tech_support: ChannelId,
    pub channel_mod_polls: ChannelId,
    pub channel_attachment_dump: Option<ChannelId>,

    pub attachment_cache_path: PathBuf,
    pub attachment_cache_max_size: usize,

    pub time_started: chrono::DateTime<chrono::Utc>,
}

impl Config {
    pub fn from_environment() -> anyhow::Result<Self> {
        Ok(Config {
            discord_token: required_env_var("TOKEN")?,
            guild: GuildId(parse_required_env_var("GUILD")?),
            role_mod: RoleId(parse_required_env_var("ROLE_MOD")?),
            role_helper: RoleId(parse_required_env_var("ROLE_HELPER")?),
            role_mute: RoleId(parse_required_env_var("ROLE_MUTE")?),
            roles_color: required_env_var("ROLES_COLOR")?
                .split(',')
                .map(|x| Ok(RoleId(x.trim().parse()?)))
                .collect::<anyhow::Result<_>>()?,
            category_mod_private: ChannelId(parse_required_env_var("CATEGORY_MOD_PRIVATE")?),
            channel_announcements: ChannelId(parse_required_env_var("CHANNEL_ANNOUNCEMENTS")?),
            channel_rules: ChannelId(parse_required_env_var("CHANNEL_RULES")?),
            channel_showcase: ChannelId(parse_required_env_var("CHANNEL_SHOWCASE")?),
            channel_feedback: ChannelId(parse_required_env_var("CHANNEL_FEEDBACK")?),
            channel_modlog: ChannelId(parse_required_env_var("CHANNEL_MODLOG")?),
            channel_auto_mod: ChannelId(parse_required_env_var("CHANNEL_AUTO_MOD")?),
            channel_mod_bot_stuff: ChannelId(parse_required_env_var("CHANNEL_MOD_BOT_STUFF")?),
            channel_bot_messages: ChannelId(parse_required_env_var("CHANNEL_BOT_MESSAGES")?),
            channel_bot_traffic: ChannelId(parse_required_env_var("CHANNEL_BOT_TRAFFIC")?),
            channel_tech_support: ChannelId(parse_required_env_var("CHANNEL_TECH_SUPPORT")?),
            channel_mod_polls: ChannelId(parse_required_env_var("CHANNEL_MOD_POLLS")?),
            channel_attachment_dump: parse_required_env_var("CHANNEL_ATTACHMENT_DUMP")
                .map(ChannelId)
                .ok(),
            attachment_cache_path: parse_required_env_var("ATTACHMENT_CACHE_PATH")?,
            attachment_cache_max_size: parse_required_env_var("ATTACHMENT_CACHE_MAX_SIZE")?,
            time_started: chrono::Utc::now(),
        })
    }

    pub async fn log_bot_action<F>(&self, ctx: &client::Context, build_embed: F)
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync,
    {
        let result = self.guild.send_embed(ctx, self.channel_modlog, build_embed).await;

        log_error!(result);
    }
    pub async fn log_automod_action<F>(&self, ctx: &client::Context, build_embed: F)
    where
        F: FnOnce(&mut CreateEmbed) + Send + Sync,
    {
        let result = self.guild.send_embed(ctx, self.channel_auto_mod, build_embed).await;
        log_error!(result);
    }

    //#[allow(unused)]
    //pub async fn is_mod(&self, ctx: &client::Context, user_id: UserId) -> Result<bool> {
    //let user = user_id.to_user(&ctx).await?;
    //Ok(user.has_role(&ctx, self.guild, self.role_mod).await?)
    //}
}

impl TypeMapKey for Config {
    type Value = Arc<Config>;
}

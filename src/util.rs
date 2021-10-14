pub mod discord_embeds {
    use chrono::Utc;
    use serenity::{builder::CreateEmbed, utils::Color};

    pub fn default_embed() -> CreateEmbed {
        CreateEmbed::default()
            .timestamp(Utc::now().to_rfc3339())
            .to_owned()
    }

    pub fn colored_default_embed() -> CreateEmbed {
        default_embed()
            .color(Color::from_rgb(179, 159, 159))
            .to_owned()
    }

    pub fn default_success_embed() -> CreateEmbed {
        default_embed()
            .color(Color::from_rgb(50, 173, 240))
            .to_owned()
    }

    pub fn default_failure_embed() -> CreateEmbed {
        default_embed()
            .color(Color::from_rgb(245, 93, 93))
            .to_owned()
    }
}

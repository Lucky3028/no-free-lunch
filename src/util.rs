use chrono::Utc;
use serenity::{builder::CreateEmbed, utils::Color};

pub trait DiscordEmbedExt {
    fn time_footer(&mut self) -> CreateEmbed;
    fn default_color(&mut self) -> CreateEmbed;
    fn success_color(&mut self) -> CreateEmbed;
    fn failure_color(&mut self) -> CreateEmbed;
}

impl DiscordEmbedExt for CreateEmbed {
    fn time_footer(&mut self) -> CreateEmbed {
        self.timestamp(Utc::now().to_rfc3339()).to_owned()
    }
    fn default_color(&mut self) -> CreateEmbed {
        self.color(Color::from_rgb(179, 159, 159)).to_owned()
    }
    fn success_color(&mut self) -> CreateEmbed {
        self.color(Color::from_rgb(50, 173, 240)).to_owned()
    }
    fn failure_color(&mut self) -> CreateEmbed {
        self.color(Color::from_rgb(245, 93, 93)).to_owned()
    }
}

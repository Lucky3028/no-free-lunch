use std::{env, sync::Arc};

use chrono::Utc;
use serenity::{async_trait, Client, model::{
    channel::Message,
    gateway::Ready,
    id::ChannelId,
}, prelude::{
    Context, EventHandler, TypeMapKey,
}, utils::Color};
use tokio::sync::RwLock;

pub struct Config {
    pub honey_pot_chs: Vec<ChannelId>,
    pub ng_words: Vec<String>,
}

impl Config {
    fn new() -> Self {
        Config {
            honey_pot_chs: Vec::new(),
            ng_words: Vec::new(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let channel_ids: Vec<u64> = vec!(896694513031086112, 896694568265863178, 896694584808198154);
        Config {
            honey_pot_chs: channel_ids.iter().map(|id| ChannelId::from(*id)).collect(),
            ng_words: vec!("free", "nitro").iter().map(|str| str.to_string().to_lowercase()).collect(),
        }
    }
}

struct Configs;

impl TypeMapKey for Configs {
    type Value = Arc<RwLock<Config>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, fired_msg: Message) {
        let guild_id = match fired_msg.guild_id {
            Some(id) => id,
            None => { return; }
        };
        if fired_msg.author.bot { return; }
        let is_spam = {
            let data = ctx.data.read().await;
            let data = data.get::<Configs>().expect("aa").clone();
            let data = data.read().await;
            let mut channels: Vec<&ChannelId> = Vec::new();
            for id in data.honey_pot_chs.iter() {
                let ch_msgs = id.messages(&ctx.http, |retriver| retriver.limit(20)).await.unwrap_or_default();
                let is_sent_by_same_author = !ch_msgs.iter().filter(|msg| msg.author == fired_msg.author).collect::<Vec<_>>().is_empty();
                let msg_contains_ng_word = ch_msgs.iter().any(|msg| data.ng_words.iter().all(|word| msg.content.to_lowercase().contains(word)));

                if is_sent_by_same_author && msg_contains_ng_word { channels.push(id) }
            }
            channels.len() == data.honey_pot_chs.len()
        };
        if !is_spam { return; }

        let _ = guild_id.ban_with_reason(&ctx.http, &fired_msg.author, 10, "Because you were considered a troll by the bot.").await;

        // ログのChに通知
        let _ = ChannelId(897488843421401130).send_message(&ctx.http, |msg| msg.embed(|embed| {
            embed.title("Troll Detected!").color(Color::from_rgb(245, 93, 93))
                .field("User", format!("{}({})", fired_msg.author.name, fired_msg.author.id), false)
                .field("Message Contents", fired_msg.content, false)
                .timestamp(Utc::now().to_rfc3339())
        })).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse::<u64>()
        .expect("The application id is not a valid id");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<Configs>(Arc::new(RwLock::new(Config::default())));
    }

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

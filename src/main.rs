use std::{env, sync::Arc};

use serenity::{
    async_trait,
    builder::CreateEmbed,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::{Context, EventHandler},
    Client,
};
use tokio::sync::RwLock;

use no_free_lunch::{util::DiscordEmbedExt, Config, GlobalConfigs};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, fired_msg: Message) {
        // DMだとguild_idはNoneになるので条件分岐
        let guild_id = match fired_msg.guild_id {
            Some(id) => id,
            None => {
                return;
            }
        };
        if fired_msg.author.bot {
            return;
        }
        let is_spam = {
            let data = ctx.data.read().await;
            let data = data.get::<GlobalConfigs>().expect("aa").clone();
            let data = data.read().await;
            let mut channels: Vec<&ChannelId> = Vec::new();
            for id in data.honey_pot_chs.iter() {
                let ch_msgs = id
                    .messages(&ctx.http, |retriver| retriver.limit(20))
                    .await
                    .unwrap_or_default();
                let is_sent_by_same_author =
                    ch_msgs.iter().any(|msg| msg.author == fired_msg.author);
                let msg_contains_ng_word = ch_msgs.iter().any(|msg| {
                    data.ng_words
                        .iter()
                        .all(|word| msg.content.to_lowercase().contains(word))
                });

                if is_sent_by_same_author && msg_contains_ng_word {
                    channels.push(id);
                }
            }
            channels.len() == data.honey_pot_chs.len()
        };
        if !is_spam {
            return;
        }

        let mut embed = match guild_id
            .ban_with_reason(
                &ctx.http,
                &fired_msg.author,
                10, // この数字日数分過去のメッセージが削除される // TODO: 戻す
                "Because you were considered a troll by the bot.",
            )
            .await
        {
            Ok(_) => CreateEmbed::default()
                .description("Succeeded in banning the user.")
                .success_color(),
            Err(err) => CreateEmbed::default()
                .description(format!("Failed to ban the user. Error: {}", err))
                .failure_color(),
        };

        // ログのChに通知
        let _ = ChannelId(897488843421401130)
            .send_message(&ctx.http, |msg| {
                let embed = embed
                    .title("Troll Detected!")
                    .field(
                        "User",
                        format!("{} ({})", fired_msg.author.tag(), fired_msg.author.id),
                        false,
                    )
                    .field("Message Contents", fired_msg.content, false)
                    .time_footer();
                msg.set_embed(embed)
            })
            .await;
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

        data.insert::<GlobalConfigs>(Arc::new(RwLock::new(Config::default())));
    }

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

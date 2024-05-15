use dotenv::dotenv;
use regex::Regex;
use serenity::all::EditMessage;
use serenity::model::prelude::Message;
use std::env;

use serenity::async_trait;
use serenity::prelude::*;

struct Bot {
    db: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, mut msg: Message) {
        let re = Regex::new(r"\s?(?:https:|http:)//(?:www\.)?(?:twitter|x)\.com(/[^\s]*)?")
            .unwrap_or_else(|e| panic!("Failed to compile regex: {}", e));
        match re.captures(&msg.content) {
            Some(captures) => {
                let timestamp = msg.timestamp.to_string();
                let user_id = msg.author.id.to_string();
                let url = msg.link();
                sqlx::query!(
                    "
                INSERT INTO twat (date, user_id, url)
                VALUES (?1, ?2, ?3)
                ",
                    timestamp,
                    user_id,
                    url,
                )
                .execute(&self.db)
                .await
                .unwrap_or_else(|e| panic!("Failed to update twat leaderboard: {}", e));
                println!(
                    "{}",
                    captures
                        .get(1)
                        .unwrap_or_else(|| panic!("No capture group found"))
                        .as_str()
                );
                match captures.get(1) {
                    Some(slug) => {
                        let channel = msg
                            .channel(&ctx.http)
                            .await
                            .unwrap_or_else(|e| panic!("Failed to get channel: {}", e))
                            .guild()
                            .unwrap_or_else(|| panic!("Failed to get guild"));
                        channel
                            .say(&ctx.http, format!("https://fxtwitter.com{}", slug.as_str()))
                            .await
                            .ok();
                        let _ = &msg
                            .edit(&ctx, EditMessage::new().suppress_embeds(true))
                            .await
                            .unwrap_or_else(|e| panic!("Failed to edit message: {}", e));
                    }
                    None => {}
                }
            }
            None => {}
        }
        if msg.content == "twats!" {
            leaderboard(self, &ctx, &msg)
                .await
                .unwrap_or_else(|e| panic!("Failed to display leaderboard: {}", e));
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Missing environment variable DATABASE_URL");
    let discord_token =
        env::var("DISCORD_TOKEN").expect("Missing environment variable DISCORD_TOKEN");
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to database");

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(discord_token, intents)
        .event_handler(Bot { db })
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn leaderboard(bot: &Bot, ctx: &Context, msg: &Message) -> serenity::Result<()> {
    let rows = sqlx::query!(
        "SELECT user_id, COUNT(user_id) AS frequency
        FROM twat
        GROUP BY user_id
        ORDER BY frequency DESC
        LIMIT 5
        ",
    )
    .fetch_all(&bot.db)
    .await
    .unwrap_or_else(|e| {
        panic!("Failed to select users for query: {}", e);
    });
    let mut message = "".to_string();
    let user = msg.member(&ctx.http).await.unwrap_or_else(|e| {
        panic!("Failed to get user: {}", e);
    });
    for record in rows {
        let nickname = user.mention();
        message.push_str(&format!("{}: {}\n", nickname, record.frequency.to_string()));
    }
    msg.reply(ctx, message).await.ok();
    Ok(())
}

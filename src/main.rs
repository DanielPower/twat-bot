use dotenv::dotenv;
use regex::Regex;
use serenity::model::prelude::Message;
use std::env;

use serenity::async_trait;
use serenity::prelude::*;

struct Bot {
    db: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let re =
            Regex::new(r"\s?(?:https:|http:)//(?:www\.)?(?:twitter|x)\.com(/[^\s]*)?").unwrap();
        match re.captures(&msg.content) {
            Some(foo) => {
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
                .unwrap();
                println!("{}", foo.get(1).unwrap().as_str());
                match foo.get(1) {
                    Some(slug) => {
                        let channel = msg.channel(&ctx.http).await.unwrap().guild().unwrap();
                        channel
                            .say(&ctx.http, format!("https://nitter.net{}", slug.as_str()))
                            .await
                            .ok();
                    }
                    None => {}
                }
            }
            None => {}
        }
        if msg.content == "twats!" {
            leaderboard(self, &ctx, &msg).await.unwrap();
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
    .unwrap();
    let mut message = "".to_string();
    let guild = msg.guild(&ctx).unwrap();
    for record in rows {
        let user_id = record.user_id.parse::<u64>().unwrap();
        let user = guild.member(&ctx, user_id).await.unwrap();
        let nickname = user.mention();
        message.push_str(&format!("{}: {}\n", nickname, record.frequency.to_string()));
    }
    msg.reply(ctx, message).await.ok();
    Ok(())
}

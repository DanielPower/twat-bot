use dotenv::dotenv;
use regex::Regex;
use serenity::all::EditMessage;
use serenity::model::prelude::Message;
use std::env;

use serenity::async_trait;
use serenity::prelude::*;

struct Bot {}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, mut msg: Message) {
        let re = Regex::new(r"\s?(?:https:|http:)//(?:www\.)?(?:twitter|x)\.com(/[^\s]*)?")
            .unwrap_or_else(|e| panic!("Failed to compile regex: {}", e));
        match re.captures(&msg.content) {
            Some(captures) => match captures.get(1) {
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
            },
            None => {}
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token =
        env::var("DISCORD_TOKEN").expect("Missing environment variable DISCORD_TOKEN");

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(discord_token, intents)
        .event_handler(Bot {})
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

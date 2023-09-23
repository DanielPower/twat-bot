use dotenv::dotenv;
use regex::Regex;
use serenity::model::prelude::Message;
use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::*;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let re = Regex::new(r"\s?(https:|http:)//(www\.)?(twitter|x)\.com(/[^\s]*)?").unwrap();
        println!("Received message: {}", msg.content);
        if re.is_match(&msg.content) {
            msg.reply_ping(&ctx.http, "Fuck Elon").await.ok();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new().group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

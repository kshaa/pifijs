use std::env;
use std::path::PathBuf;
// use std::process::Command;

use serenity::all::{CreateAttachment, CreateMessage};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tokio::process::Command;

struct Handler;

async fn render_image(image_path: &PathBuf) {
    let renderer = env::var("PIFIJS_RENDERER").expect("Expected PIFIJS_RENDERER in the environment");
    let result = Command::new(renderer)
        .arg(&image_path)
        .output()
        .await;
    let output = result.expect("Renderer seems to have failed");
    let stdout = std::str::from_utf8(output.stdout.as_ref()).unwrap();
    let stderr = std::str::from_utf8(output.stderr.as_ref()).unwrap();
    println!("{}", stdout);
    println!("{}", stderr);
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!image" {
            let image_path = PathBuf::from("/tmp/test.png");
            render_image(&image_path).await;
            let attachment = CreateAttachment::path(&image_path).await;
            std::fs::remove_file(&image_path).unwrap();
            match attachment {
                Err(why) => println!("Error reading file message: {why:?}"),
                Ok(content) => {
                    if let Err(why) = msg.channel_id.send_message(&ctx.http, CreateMessage::new().add_file(content)).await {
                        println!("Error sending message: {why:?}")
                    }
                }
            }
        } else if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

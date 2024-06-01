pub mod plotter;
pub mod message;
use std::env;
use std::path::PathBuf;
use message::PifijsMessage;
use plotter::render_plot;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;
impl Handler {
    pub async fn respond_text(ctx: Context, msg: Message, text: String) {
        if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
            eprintln!("Error sending message: {why:?}");
        }    
    }

    pub async fn respond_ping(ctx: Context, msg: Message) {
        Handler::respond_text(ctx, msg, String::from("pong!")).await
    }

    pub async fn respond_plot(ctx: Context, msg: Message, plot_request: String) {
        let id = msg.id.get();
        let image_path = PathBuf::from(format!("/tmp/{}.png", id));
        
        render_plot(&image_path, &plot_request).await;
        let attachment = CreateAttachment::path(&image_path).await;
        std::fs::remove_file(&image_path).unwrap();
        match attachment {
            Err(why) => eprintln!("Error reading file: {why:?}"),
            Ok(content) => {
                if let Err(why) = msg.channel_id.send_message(&ctx.http, CreateMessage::new().add_file(content)).await {
                    eprintln!("Error sending message: {why:?}")
                }
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let parsed = PifijsMessage::parse(msg.content.clone());

        if let Some(Ok(message)) = parsed {
            match message {
                PifijsMessage::Ping() => Handler::respond_ping(ctx, msg).await,
                PifijsMessage::Plot(plot_request) => Handler::respond_plot(ctx, msg, plot_request).await,
            }
        } else if let Some(Err(why)) = parsed {
            Handler::respond_text(ctx, msg, why).await
        };
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

use serenity::all::Reaction;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        reaction.channel_id.say(&_ctx.http, format!("{} reacted", reaction.emoji)).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let token = "MTI5OTg5OTkyMzcwOTQyNzczMg.GYLAdE.7auvKEkHnvQfuJmJnbCcgHP2dy4W3ryfBuB9Go";

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
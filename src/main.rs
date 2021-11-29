use std::{
    env,
};

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler,bridge::gateway::GatewayIntents, validate_token},
    model::{channel::Message, prelude::*},
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    },
    http::Http,
};

#[group]
#[commands(ching)]
struct General;
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("connected to Discord as {}", ready.user.name);
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        let activity = Activity::watching("Satan Approved");
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status).await;//TODO: maybe make status cycle (should check Discord's status update ratelimit first)
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
    async fn guild_member_addition(&self, _ctx: Context, _guild_id: serenity::model::id::GuildId, mut _new_member: serenity::model::guild::Member) {
        println!("{name} has joined", name=&_new_member.user.name);
    }
}

#[tokio::main]
async fn main() {
    let bot_token = env::var("SLUR_COUNTER_BOT_TOKEN").expect("Expected a token in the environment");
    assert!(validate_token(&bot_token).is_ok());

    let http = Http::new_with_token(&bot_token);

    let bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix("`") // set the bot's prefix to "`"
            .on_mention(Some(bot_id))
            .owners(vec![UserId(386699245240975371)].into_iter().collect())
            .allow_dm(true)
            .case_insensitivity(false)
            .no_dm_prefix(true)
            })
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(bot_token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    let _slurs_to_track: Vec<&str> = vec!["nigger", "kike", "coon", "spic", "chink", "faggot", "tranny"];
}

#[command]
async fn ching(ctx: &Context, msg: &Message) -> CommandResult {
    println!("replied to a message");
    msg.reply(ctx, "chong!").await?;

    Ok(())
}
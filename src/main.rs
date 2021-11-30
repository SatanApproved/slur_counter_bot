use std::{
    collections::{
        HashMap
    },
    env,
     sync::{
         Arc
    }
};

use serenity::{async_trait, client::{Client, Context, EventHandler,bridge::gateway::GatewayIntents, validate_token}, framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    }, http::Http, model::{channel::Message, prelude::*}, prelude::{RwLock, TypeMapKey}};

use strum::IntoEnumIterator;

use strum_macros::EnumIter;


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
    
    async fn message(&self, ctx: Context, msg: Message) {
        for variant in Slurs::iter() {
            if msg.content.contains(variant.to_str()) {
                //ensure that ctx.data's HashMap<User, SlurCountStruct> contains this user
                let count = msg.content.matches(variant.to_str()).count();
                let mut scs;
                match ctx.data.read().await.get::<SlurCountStruct>().expect("expected to find a SlurCountStruct in ctx.data").read().await.get(&msg.author) {
                    None => {
                        scs = SlurCountStruct::default();
                        match variant {
                            Slurs::NIGGER => scs.nigger = count,
                            Slurs::KIKE => scs.kike = count,
                            Slurs::COON => scs.coon = count,
                            Slurs::SPIC => scs.spic = count,
                            Slurs::CHINK => scs.chink = count,
                            Slurs::FAGGOT => scs.faggot = count,
                            Slurs::TRANNY => scs.tranny = count,
                        }
                    },
                    Some(s) => {
                        scs = s.clone();
                        match variant {
                            Slurs::NIGGER => scs.nigger += count,
                            Slurs::KIKE => scs.kike += count,
                            Slurs::COON => scs.coon += count,
                            Slurs::SPIC => scs.spic += count,
                            Slurs::CHINK => scs.chink += count,
                            Slurs::FAGGOT => scs.faggot += count,
                            Slurs::TRANNY => scs.tranny += count,
                        }
                    }
                }
                ctx.data.write().await.get::<SlurCountStruct>().expect("expected to find a SlurCountStruct in ctx.data").write().await.insert(msg.author.clone(), scs);
            }
        }
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
}

#[command]
async fn ching(ctx: &Context, msg: &Message) -> CommandResult {
    println!("replied to a message");
    msg.reply(ctx, "chong!").await?;

    Ok(())
}
#[non_exhaustive]
#[derive(EnumIter)]
enum Slurs {
    NIGGER,
    KIKE,
    COON,
    SPIC,
    CHINK,
    FAGGOT,
    TRANNY,
}
impl Slurs {
    fn to_str(&self) -> &str {
        match self {
            Self::NIGGER => "nigger",
            Self::KIKE => "kike",
            Self::COON => "coon",
            Self::SPIC => "spic",
            Self::CHINK => "chink",
            Self::FAGGOT => "faggot",
            Self::TRANNY => "tranny",
        }
    }
}
#[derive(Default, Clone)]
struct SlurCountStruct {
    nigger: usize,
    kike: usize,
    coon: usize,
    spic: usize,
    chink: usize,
    faggot: usize,
    tranny: usize,
}
impl TypeMapKey for SlurCountStruct {
    type Value = Arc<RwLock<HashMap<User, SlurCountStruct>>>;
}
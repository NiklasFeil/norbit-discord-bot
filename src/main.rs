mod file_reader;

use file_reader::*;

use poise::serenity_prelude as serenity;

use serenity::prelude::*;

use songbird::events::TrackEvent;
use songbird::SerenityInit;
use songbird::*;

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the json.
    let token = DiscordBotInformationHandler::new("sensitive_information.json").get_bot_token();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MESSAGES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![join(), leave(), play()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[poise::command(prefix_command, slash_command)]
async fn join(ctx: Context<'_>) -> Result<(), Error> {
    println!("Executed join command");
    //let channel_id = ctx.channel_id();

    let (guild_id, channel_id) = {
        let guild_id = ctx.guild_id().ok_or("Command not in a guild")?;
        let user_id = ctx.author().id;

        let guild = match guild_id.to_guild_cached(&ctx.serenity_context().cache) {
            Some(guild) => guild,
            None => {
                println!("Command not written on server. Interrupt join command.");
                return Ok(());
            }
        };

        let voice_state = match guild.voice_states.get(&user_id) {
            Some(state) => state,
            None => {
                println!("Voice state of user could not be received. Interrupt join command.");
                return Ok(());
            }
        };
        (guild_id, voice_state.channel_id)
    };

    let channel_to_connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            println!("User is not in a voice channel. Interrupt join command.");
            ctx.say("Not in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    match manager.join(guild_id, channel_to_connect_to).await {
        Err(why) => {
            println!("Joining channel failed. This is why: {:?}", why);
            return Ok(());
        }
        _ => {
            ctx.say("Joining channel...").await?;
        }
    };

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    println!("Executing leave command");

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return Ok(()),
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialization when trying to leave.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            ctx.say(format!("Failed: {:?}", e)).await?;
            return Ok(());
        };

        ctx.say("Leaving channel...").await?;
    } else {
        ctx.say("I'm not in a voice channel").await?;
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn play(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

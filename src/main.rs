use arson_bot::command::duel::duel;
use arson_bot::*;
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: CmdCtx<'_>) -> Res {
    ctx.say("Hello").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("ARSON_TOKEN").expect("Missing token");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), duel()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

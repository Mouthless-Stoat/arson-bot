use poise::serenity_prelude::{
    ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
};
use poise::{command, CreateReply};

use crate::{CmdCtx, Deck, InteractionAdapter, Player, Res};

#[command(slash_command)]
pub async fn play_uno(ctx: CmdCtx<'_>) -> Res {
    let uuid = ctx.id();

    let mut players = vec![(
        ctx.author().clone(),
        match ctx {
            CmdCtx::Application(ctx) => InteractionAdapter::Command(ctx.interaction.clone()),
            _ => unreachable!(),
        },
    )];

    let reply = {
        let components = vec![CreateActionRow::Buttons(vec![CreateButton::new(format!(
            "join_game_{uuid}"
        ))
        .style(ButtonStyle::Primary)
        .label("Join Game")])];

        CreateReply::default()
            .content("Join the game! Player: 1")
            .components(components)
    };

    let reply = ctx.send(reply).await?;

    while let Some(mci) = ComponentInteractionCollector::new(ctx)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(5))
        .filter(move |mci| mci.data.custom_id.contains(&uuid.to_string()))
        .await
    {
        if !players.iter().any(|i| i.0 == mci.user) {
            players.push((mci.user.clone(), InteractionAdapter::Component(mci.clone())));
        }

        mci.create_response(
            ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .content(format!("Join the game! Player: {}", players.len())),
            ),
        )
        .await?;
    }

    reply
        .edit(
            ctx,
            CreateReply::default()
                .content("Too late game will start shortly")
                .components(vec![]),
        )
        .await?;

    let mut deck = Deck::new();

    let players = {
        let mut t = vec![];
        for (user, interaction) in players {
            t.push(Player {
                hand: deck.draw_cards(7).expect("Deck shouldn't be empty"),
                user,
                interaction,
            });
        }
        t
    };

    let running = true;
    let curr_player_index = 0;
    let clock_wise = true;

    let curr_player = &players[curr_player_index];

    curr_player
        .interaction
        .followup_ephemeral(
            ctx,
            CreateInteractionResponseFollowup::new().content("Hello"),
        )
        .await?;

    Ok(())
}

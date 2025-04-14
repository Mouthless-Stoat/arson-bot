use std::time::Duration;

use poise::serenity_prelude::{
    collector, ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, User,
};
use poise::{command, CreateReply};
use rand::{thread_rng, Rng};

use crate::{CmdCtx, Res};

#[command(slash_command)]
pub async fn duel(
    ctx: CmdCtx<'_>,
    #[description = "Opponent you want to duel"] opponent: User,
    width: Option<usize>,
    height: Option<usize>,
) -> Res {
    let width = width.unwrap_or(5);
    let height = height.unwrap_or(3);

    let reply = {
        let component = vec![CreateActionRow::Buttons(vec![
            CreateButton::new("accept")
                .label("Accept")
                .style(ButtonStyle::Success),
            CreateButton::new("decline")
                .label("Decline")
                .style(ButtonStyle::Danger),
        ])];

        CreateReply::default()
            .content(format!(
                "<@{}>. DO you wish to duel <@{}>?\nBoard Dimension: {}x{}",
                opponent.id,
                ctx.author().id,
                width,
                height
            ))
            .components(component)
    };

    let msg = ctx.send(reply).await?;

    let collector = ComponentInteractionCollector::new(ctx)
        .author_id(opponent.id)
        .timeout(Duration::from_secs(30));

    let mut decline = false;

    if let Some(res) = collector.await {
        if res.data.custom_id == "accept" {
            res.create_response(
                ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .content("Duel accepted waiting to start...")
                        .components(vec![]),
                ),
            )
            .await?;
        } else {
            decline = true;
            res.create_response(
                ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .content("Opponent canceled")
                        .components(vec![]),
                ),
            )
            .await?;
        }
    } else {
        msg.edit(
            ctx,
            CreateReply::default()
                .content("Request timed out")
                .components(vec![]),
        )
        .await?;
    }

    if decline {
        return Ok(());
    }

    let players = [ctx.author().id, opponent.id];

    let mut components = vec![];

    let shot_slot = (
        thread_rng().random_range(0..height),
        thread_rng().random_range(0..width),
    );

    for i in 0..height {
        let mut btns = vec![];
        for j in 0..width {
            btns.push(if i == shot_slot.0 && j == shot_slot.1 {
                CreateButton::new("shot")
                    .label("Shot")
                    .style(ButtonStyle::Secondary)
            } else {
                CreateButton::new(format!("shit{}{}", i, j))
                    .label("Shit")
                    .style(ButtonStyle::Secondary)
            });
        }
        components.push(CreateActionRow::Buttons(btns));
    }

    msg.edit(
        ctx,
        CreateReply::default()
            .content("FIGHT!")
            .components(components),
    )
    .await?;

    let collector = ComponentInteractionCollector::new(ctx)
        .filter(move |i| players.contains(&i.user.id) && i.data.custom_id == "shot");

    if let Some(interaction) = collector.await {
        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .content(format!("<@{}> Win!", interaction.user.id))
                        .components(vec![]),
                ),
            )
            .await?;
    } else {
        msg.edit(
            ctx,
            CreateReply::default()
                .content("Both players timed out.")
                .components(vec![]),
        )
        .await?;
    }

    Ok(())
}

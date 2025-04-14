use poise::serenity_prelude::{
    CacheHttp, CommandInteraction, ComponentInteraction, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, User,
};

use self::card::{Card, Color, Value};

pub mod card;
pub mod command;

pub struct Data {}

/// Discord bot error type alias.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
/// Poise context type alias.
pub type CmdCtx<'a> = poise::Context<'a, Data, Error>;

/// Discord bot function return type.
pub type Res = Result<(), Error>;

pub struct Player {
    hand: Vec<Card>,
    user: User,
    interaction: InteractionAdapter,
}

pub struct Deck(Vec<Card>);

impl Deck {
    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn draw_cards(&mut self, amount: usize) -> Option<Vec<Card>> {
        let mut out = vec![];
        for _ in 0..amount {
            out.push(self.0.pop()?);
        }
        Some(out)
    }

    pub fn new() -> Self {
        let mut deck = vec![];
        for color in [Color::Red, Color::Yellow, Color::Blue, Color::Green] {
            deck.push(Card::new(Value::Num(0), color));
            for _ in 0..2 {
                for num in 1..=9 {
                    deck.push(Card::new(Value::Num(num), color));
                }
                deck.push(Card::new(Value::Draw(2), color));
                deck.push(Card::new(Value::Reverse, color));
                deck.push(Card::new(Value::Skip, color));
            }
            deck.push(Card::new(Value::None, Color::Wild));
            deck.push(Card::new(Value::Draw(4), Color::Wild));
        }
        Deck(deck)
    }
}

pub enum InteractionAdapter {
    Command(CommandInteraction),
    Component(ComponentInteraction),
}

macro_rules! interaction_adapter_impl {
    ($(fn $func:ident($($args:ident: $type:ty),*) {let $id:ident, $ctx:ident; $($tt:tt)*})*) => {
        impl InteractionAdapter {
            $(
                pub async fn $func(&self, ctx: impl CacheHttp, $($args: $type),*) -> Res {
                    let $ctx = ctx;
                    match self {
                        InteractionAdapter::Command($id) => {
                            $($tt)*
                        }
                        InteractionAdapter::Component($id) => {
                            $($tt)*
                        }
                    }
                }
            )*
        }
    };
}
interaction_adapter_impl! {
    fn follow_up(msg: CreateInteractionResponseFollowup) {
        let inter, ctx;
        inter.create_followup(ctx,msg).await?;
        Ok(())
    }

    fn followup_ephemeral(msg: CreateInteractionResponseFollowup) {
        let inter, ctx;
        inter.create_followup(ctx,msg.ephemeral(true)).await?;
        Ok(())
    }

    fn edit(msg: CreateInteractionResponseMessage) {
        let inter, ctx;
        inter.create_response(ctx, CreateInteractionResponse::UpdateMessage(msg)).await?;
        Ok(())
    }

}

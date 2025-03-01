import hikari
import csv
import random
import os

bot = hikari.GatewayBot(
    token=os.environ["ARSON_TOKEN"]
)

riddles = {}

with open("./riddle.csv") as file:
    rows = csv.reader(file, delimiter=",", quotechar='"')
    for row in rows:
        riddles[row[0]] = row[1]


@bot.listen()
async def ping(event: hikari.GuildMessageCreateEvent) -> None:
    """If a non-bot user mentions your bot, respond with 'Pong!'."""

    # Do not respond to bots nor webhooks pinging us, only user accounts
    if not event.is_human:
        return

    me = bot.get_me()

    if me.id in event.message.user_mentions_ids:
        question, answer = random.choice(list(riddles.items()))
        await event.message.respond(f"{question}\nAnswer: ||{"NO PEEKING >:(" if random.randint(1,3) == 1 else answer }||")


bot.run()

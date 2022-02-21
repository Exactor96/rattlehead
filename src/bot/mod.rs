// This bot throws a dice on each incoming message.

use teloxide::prelude2::*;

pub async fn start_bot() {

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        bot.send_dice(message.chat.id).await?;
        respond(())
    })
    .await;
}

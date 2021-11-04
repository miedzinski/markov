use serenity::Client;
use tokio::sync::mpsc;

use super::command::MessageCommand;
use super::handler::Handler;
use crate::markov::bot::Bot;
use crate::markov::choose::Choose;
use crate::markov::repository::Repository;
use crate::markov::shuffle::Shuffle;

pub struct DiscordBot;

impl DiscordBot {
    pub fn new() -> DiscordBot {
        DiscordBot
    }

    pub async fn run<const N: usize>(
        &self,
        bot: Bot<
            impl Repository<String, N> + Send + 'static,
            impl Choose<String> + Send + 'static,
            impl Shuffle<String> + Send + 'static,
            N,
        >,
        token: &str,
        verbosity: f64,
    ) -> anyhow::Result<()> {
        let (sender, receiver): (mpsc::Sender<MessageCommand>, mpsc::Receiver<MessageCommand>) =
            mpsc::channel(32);
        let handler = Handler::new(verbosity, sender);
        let mut client = Client::builder(&token).event_handler(handler).await?;

        tokio::spawn(handle_messages(bot, receiver));
        client.start().await.map_err(Into::into)
    }
}

async fn handle_messages<const N: usize>(
    mut bot: Bot<
        impl Repository<String, N> + Send,
        impl Choose<String> + Send,
        impl Shuffle<String> + Send,
        N,
    >,
    mut receiver: mpsc::Receiver<MessageCommand>,
) -> anyhow::Result<()> {
    while let Some(cmd) = receiver.recv().await {
        let content = &cmd.content;

        bot.learn(content)?;

        if cmd.should_reply {
            let reply = bot.reply(content);
            if let Ok(reply) = reply {
                let _ = cmd.sender.send(Some(reply));
            }
        }
    }

    Ok(())
}

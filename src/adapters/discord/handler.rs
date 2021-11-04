use rand::{thread_rng, Rng};
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::misc::Mentionable;
use tokio::sync::{mpsc, oneshot};

use crate::adapters::discord::command::MessageCommand;

pub struct Handler {
    verbosity: f64,
    sender: mpsc::Sender<MessageCommand>,
}

impl Handler {
    pub fn new(verbosity: f64, sender: mpsc::Sender<MessageCommand>) -> Handler {
        Handler { verbosity, sender }
    }

    async fn should_reply(&self, ctx: &Context, msg: &Message) -> bool {
        if msg.mentions_user_id(ctx.cache.current_user_id().await) {
            return true;
        }
        let mut rng = thread_rng();
        let random: f64 = rng.gen();
        random < self.verbosity
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user_id().await {
            return;
        }

        let (reply_sender, reply_receiver) = oneshot::channel();
        let should_reply = self.should_reply(&ctx, &msg).await;
        let content = msg
            .content
            .strip_prefix(&ctx.cache.current_user().await.mention().to_string())
            .map(str::to_string)
            .unwrap_or(msg.content);
        let command = MessageCommand {
            content,
            should_reply,
            sender: reply_sender,
        };

        let _ = self.sender.send(command).await;

        if should_reply {
            let reply = reply_receiver.await.unwrap();

            if let Some(reply) = reply {
                msg.channel_id.say(&ctx.http, reply).await.unwrap();
            }
        }
    }
}

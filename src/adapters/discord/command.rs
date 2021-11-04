use tokio::sync::oneshot;

pub struct MessageCommand {
    pub content: String,
    pub should_reply: bool,
    pub sender: oneshot::Sender<Option<String>>,
}

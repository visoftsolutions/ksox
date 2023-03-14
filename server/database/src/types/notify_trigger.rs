use tokio::sync::oneshot;

pub struct NotifyTrigger {
    pub channel_name: String,
    drop_signal: Option<oneshot::Sender<()>>,
}

impl NotifyTrigger {
    pub fn new(channel_name: String, drop_signal: oneshot::Sender<()>) -> Self {
        Self {
            drop_signal: Some(drop_signal),
            channel_name,
        }
    }
}
impl Drop for NotifyTrigger {
    fn drop(&mut self) {
        if let Some(drop_signal) = self.drop_signal.take() {
            if drop_signal.send(()).is_err() {
                tracing::error!("drop signal failed");
            }
        }
    }
}

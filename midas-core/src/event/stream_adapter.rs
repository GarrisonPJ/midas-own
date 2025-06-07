use super::{Event, ParallelEventHandler};
use crate::stream::DataStream;
use std::sync::Arc;

pub struct StreamAdapter {
    handler: Arc<dyn ParallelEventHandler>,
}

impl StreamAdapter {
    pub fn new(handler: Arc<dyn ParallelEventHandler>) -> Self {
        Self { handler }
    }

    pub async fn process_stream(&self, mut stream: DataStream<Event>) {
        while let Some(event) = stream.next().await {
            self.handler.handle(&event).await;
        }
    }
}
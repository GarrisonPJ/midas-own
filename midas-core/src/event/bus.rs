use tokio::sync::mpsc;
use super::Event;
use std::sync::Arc;

// 新增事件处理器trait
pub trait ParallelEventHandler: Send + Sync {
    async fn handle(&self, event: &Event);
}

pub struct EventBus {
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Event>,
    handlers: Vec<Arc<dyn ParallelEventHandler>>, // 新增处理器集合
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel(capacity);
        Self { 
            tx, 
            rx,
            handlers: Vec::new(), // 初始化空处理器集合
        }
    }

    pub fn sender(&self) -> mpsc::Sender<Event> {
        self.tx.clone()
    }

    pub fn receiver(&mut self) -> &mut mpsc::Receiver<Event> {
        &mut self.rx
    }

    // 新增处理器注册方法
    pub fn register_handler(&mut self, handler: Arc<dyn ParallelEventHandler>) {
        self.handlers.push(handler);
    }

    // 新增事件分发方法
    pub async fn dispatch(&mut self) {
        while let Some(event) = self.rx.recv().await {
            for handler in &self.handlers {
                handler.handle(&event).await;
            }
        }
    }
}
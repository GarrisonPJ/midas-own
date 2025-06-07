use super::{bus::EventBus, handler::{OptimizedOrderBookHandler, StrategyHandler, ParallelEventHandler}};
use tokio::task;

pub struct EventSystem {
    bus: EventBus,
}

impl EventSystem {
    pub fn new() -> Self {
        let mut bus = EventBus::new(1024);
        
        // 注册默认处理器
        bus.register_handler(Arc::new(ParallelEventHandler));
        bus.register_handler(Arc::new(ParallelEventHandler));
        
        Self { bus }
    }

    pub async fn run(&mut self) {
        let dispatch_task = task::spawn(async move {
            self.bus.dispatch().await;
        });
        
        // 这里可以添加其他系统组件的启动逻辑
        dispatch_task.await.unwrap();
    }

    pub fn get_sender(&self) -> mpsc::Sender<Event> {
        self.bus.sender()
    }
}
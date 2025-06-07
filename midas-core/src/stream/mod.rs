use tokio::sync::mpsc;
use std::pin::Pin;
use futures::Stream;

pub struct DataStream<T> {
    rx: mpsc::Receiver<T>,
    buffer_size: usize,
}

impl<T: 'static> DataStream<T> {
    pub fn new(buffer_size: usize) -> (Self, mpsc::Sender<T>) {
        let (tx, rx) = mpsc::channel(buffer_size);
        (Self { rx, buffer_size }, tx)
    }

    /// 流过滤操作
    pub fn filter<F>(self, predicate: F) -> impl Stream<Item = T>
    where
        F: Fn(&T) -> bool + Send + 'static,
    {
        self.rx.filter(predicate)
    }

    /// 流映射操作
    pub fn map<U, F>(self, f: F) -> impl Stream<Item = U>
    where
        F: Fn(T) -> U + Send + 'static,
        U: Send + 'static,
    {
        self.rx.map(f)
    }

    /// 流批处理操作
    pub fn batch(self, size: usize) -> impl Stream<Item = Vec<T>> {
        self.rx.chunks(size)
    }

    /// 流时间窗口聚合
    pub fn time_window(self, duration: std::time::Duration) -> impl Stream<Item = Vec<T>> {
        self.rx.timeout(duration).collect()
    }
}

// 为tokio::mpsc::Receiver实现扩展trait
trait StreamExt<T> {
    fn filter<F>(self, predicate: F) -> impl Stream<Item = T>;
    fn map<U, F>(self, f: F) -> impl Stream<Item = U>;
}

impl<T> Stream for DataStream<T> {
    type Item = T;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.get_mut();
        match this.rx.poll_recv(cx) {
            std::task::Poll::Ready(Some(item)) => std::task::Poll::Ready(Some(item)),
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}


pub trait Aggregator {
    type Output;
    fn apply(&mut self, window: &[Event]) -> Self::Output;
}

// 示例实现：
struct VolumeAggregator;
impl Aggregator for VolumeAggregator {
    type Output = f64;
    fn apply(&mut self, window: &[Event]) -> f64 {
        window.iter().filter_map(|e| e.volume()).sum()
    }
}
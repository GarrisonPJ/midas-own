use std::time::{Duration, Instant};
use futures::StreamExt;

struct WindowState<T> {
    data: Vec<T>,
    last_flush: Instant,
}

pub struct TimeWindowStream<S> {
    inner: S,
    duration: Duration,
    state: WindowState<S::Item>, // 合并缓冲区和时间状态
}

impl<S: Stream> TimeWindowStream<S> {
    pub fn new(inner: S, duration: Duration) -> Self {
        Self {
            inner,
            duration,
            state: WindowState {
                data: Vec::new(),
                last_flush: Instant::now(),
            },
        }
    }
}

impl<S: Stream + Unpin> Stream for TimeWindowStream<S> {
    type Item = Vec<S::Item>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        loop {
            match Pin::new(&mut self.inner).poll_next(cx) {
                std::task::Poll::Ready(Some(item)) => {
                    self.buffer.push(item);
                    if self.last_flush.elapsed() >= self.duration {
                        let window = std::mem::take(&mut self.buffer);
                        self.last_flush = Instant::now();
                        return std::task::Poll::Ready(Some(window));
                    }
                }
                std::task::Poll::Ready(None) => {
                    if !self.buffer.is_empty() {
                        let window = std::mem::take(&mut self.buffer);
                        return std::task::Poll::Ready(Some(window));
                    }
                    return std::task::Poll::Ready(None);
                }
                std::task::Poll::Pending => {
                    if !self.buffer.is_empty() && self.last_flush.elapsed() >= self.duration {
                        let window = std::mem::take(&mut self.buffer);
                        self.last_flush = Instant::now();
                        return std::task::Poll::Ready(Some(window));
                    }
                    return std::task::Poll::Pending;
                }
            }
        }
    }
}

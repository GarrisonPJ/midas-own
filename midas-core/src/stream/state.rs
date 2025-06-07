use super::window::TimeWindowStream;

pub struct StatefulStream<T, F> {
    inner: TimeWindowStream<Event>, // 固定事件类型
    state: T,
    aggregator: F,
}

impl<T, F, R> StatefulStream<T, F>
where
    F: FnMut(&mut T, Vec<Event>) -> R,
{
    pub fn new(stream: TimeWindowStream<Event>, init_state: T, aggregator: F) -> Self {
        Self { inner: stream, state: init_state, aggregator }
    }
}
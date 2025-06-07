use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct MergedStream<S1, S2> {
    stream1: S1,
    stream2: S2,
    buffered_item: Option<(bool, S1::Item)>,
}

impl<S1, S2> MergedStream<S1, S2>
where
    S1: Stream + Unpin,
    S2: Stream<Item = S1::Item> + Unpin,
{
    pub fn new(stream1: S1, stream2: S2) -> Self {
        Self {
            stream1,
            stream2,
            buffered_item: None,
        }
    }
}

impl<S1, S2> Stream for MergedStream<S1, S2>
where
    S1: Stream + Unpin,
    S2: Stream<Item = S1::Item> + Unpin,
{
    type Item = S1::Item;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if let Some((is_stream1, item)) = self.buffered_item.take() {
            return Poll::Ready(Some(item));
        }

        match self.stream1.poll_next_unpin(cx) {
            Poll::Ready(Some(item)) => {
                self.buffered_item = Some((true, item));
                match self.stream2.poll_next_unpin(cx) {
                    Poll::Ready(Some(item)) => Poll::Ready(Some(item)),
                    _ => Poll::Pending,
                }
            }
            Poll::Ready(None) => self.stream2.poll_next_unpin(cx),
            Poll::Pending => self.stream2.poll_next_unpin(cx),
        }
    }
}
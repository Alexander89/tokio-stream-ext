use core::task::Context;
use futures::StreamExt;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::Poll;
use tokio_stream::Stream;

pin_project! {
    pub struct SwitchMap<I, T, O>
    {
        #[pin]
        from: I,
        mapper: T,
        #[pin]
        mapped_stream: Option<O>,
    }
}

#[allow(dead_code)]
pub fn switch_map<I, T, O>(from: I, mapper: T) -> SwitchMap<I, T, O>
where
    I: Stream,
    T: Fn(I::Item) -> Option<O> + Clone,
    O: Stream,
{
    SwitchMap {
        from: from,
        mapper,
        mapped_stream: None,
    }
}

impl<I, T, O> Stream for SwitchMap<I, T, O>
where
    I: Stream,
    T: Fn(I::Item) -> Option<O> + Clone,
    O: Stream,
{
    type Item = O::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut me = self.project();
        while let Poll::Ready(p) = me.from.poll_next_unpin(cx) {
            if let Some(state) = p {
                me.mapped_stream.set((me.mapper)(state));
            } else {
                return Poll::Ready(None);
            }
        }
        if let Some(mut mapped) = me.mapped_stream.as_mut().as_pin_mut() {
            while let Poll::Ready(p) = mapped.poll_next_unpin(cx) {
                if let Some(state) = p {
                    return Poll::Ready(Some(state));
                } else {
                    me.mapped_stream.set(None);
                    return Poll::Pending;
                }
            }
        }

        Poll::Pending
    }
}

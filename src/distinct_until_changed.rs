use core::fmt;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{ready, Stream};
use pin_project_lite::pin_project;

pin_project! {
    /// Stream for the [`distinctUntilChanged`](super::StreamOpsExt::distinctUntilChanged) method.
    #[must_use = "streams do nothing unless polled"]
    pub struct DistinctUntilChanged<St, Item >
    where
        St: Stream<Item = Item>,
        Item: PartialEq,
    {
        #[pin]
        stream: St,
        last: Option<Item>,
    }
}

impl<St, Item> fmt::Debug for DistinctUntilChanged<St, Item>
where
    St: Stream<Item = Item> + fmt::Debug,
    Item: PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DistinctUntilChanged")
            .field("stream", &self.stream)
            .finish()
    }
}

impl<St, Item> DistinctUntilChanged<St, Item>
where
    St: Stream<Item = Item>,
    Item: PartialEq,
{
    #[allow(dead_code)]
    pub(super) fn new(stream: St) -> Self {
        DistinctUntilChanged { stream, last: None }
    }
}

impl<St, Item> Stream for DistinctUntilChanged<St, Item>
where
    St: Stream<Item = Item>,
    Item: Clone + PartialEq,
{
    type Item = St::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = self.as_mut().project();
        let next = ready!(me.stream.poll_next(cx));
        match next {
            Some(v) if me.last.is_none() => {
                *me.last = Some(v.clone());
                Poll::Ready(Some(v))
            }
            value @ Some(_) if me.last.is_some() && value != *me.last => {
                *me.last = value.clone();
                Poll::Ready(value)
            }
            Some(_) => Poll::Pending,
            None => Poll::Ready(None),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

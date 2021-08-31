use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{Future, Stream};
use pin_project_lite::pin_project;
use std::time::Duration;
use tokio::time::{Instant, Sleep};

pin_project! {
    /// Stream for the [`distinctUntilChanged`](super::StreamOpsExt::distinctUntilChanged) method.
    #[must_use = "streams do nothing unless polled"]
    pub struct Debounce<St: Stream> {
        #[pin]
        value: St,
        #[pin]
        delay: Sleep,
        #[pin]
        debounce_time: Duration,
        #[pin]
        last_state: Option<St::Item>
    }
}

impl<St> Debounce<St>
where
    St: Stream + Unpin,
{
    #[allow(dead_code)]
    pub(super) fn new(stream: St, debounce_time: Duration) -> Debounce<St> {
        Debounce {
            value: stream,
            delay: tokio::time::sleep(debounce_time),
            debounce_time,
            last_state: None,
        }
    }
}

impl<St, Item> Stream for Debounce<St>
where
    St: Stream<Item = Item>,
    Item: Clone + Unpin,
{
    type Item = St::Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut me = self.project();

        // First, try polling the stream
        match me.value.poll_next(cx) {
            Poll::Ready(Some(v)) => {
                let d = (*me.debounce_time).clone();
                me.delay.as_mut().reset(Instant::now() + (d)); // FixMe doubleing issue
                *me.last_state = Some(v);
            }
            Poll::Ready(None) => {
                let l = (*me.last_state).clone();
                *me.last_state = None;
                return Poll::Ready(l);
            }
            _ => (),
        }

        // Now check the timer
        match me.delay.poll(cx) {
            Poll::Ready(()) => {
                if let Some(l) = (*me.last_state).clone() {
                    *me.last_state = None;
                    return Poll::Ready(Some(l));
                } else {
                    Poll::Pending
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.value.size_hint()
    }
}

#[cfg(test)]
mod test {
    use crate::StreamOpsExt;
    use std::time::Duration;
    use tokio::{sync::mpsc, time::sleep};
    use tokio_stream::{wrappers::ReceiverStream, StreamExt};

    #[tokio::test]
    async fn debounce_test() {
        let (tx, rx) = mpsc::channel(5);
        let j = tokio::spawn(async move {
            for i in 1..4 {
                sleep(Duration::from_millis(100 * i)).await;
                tx.send(i).await.unwrap();
            }
        });

        let mut stream = Box::pin(ReceiverStream::new(rx).debounce(Duration::from_millis(250)));

        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, None);
        assert!(j.await.is_ok());
    }
}

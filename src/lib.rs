use futures::Stream;
use std::time::Duration;

mod combine_latest;
pub use combine_latest::{combine_latest, CombineLatest};

mod switch_map;
pub use switch_map::{switch_map, SwitchMap};

mod distinct_until_changed;
pub use distinct_until_changed::DistinctUntilChanged;

mod debounce;
pub use debounce::Debounce;

pub trait StreamOpsExt: Stream {
    fn distinct_until_changed<Item>(self) -> DistinctUntilChanged<Self, Item>
    where
        Self: Stream<Item = Item> + Sized,
        Item: PartialEq,
    {
        DistinctUntilChanged::new(self)
    }

    fn debounce(self, debounce_time: Duration) -> Debounce<Self>
    where
        Self: Sized + Unpin,
    {
        Debounce::new(self, debounce_time)
    }
}

impl<T: ?Sized> StreamOpsExt for T where T: Stream {}

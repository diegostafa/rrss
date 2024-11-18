use std::cmp::Ordering;

use super::models::{Feed, Item, Link, Tag};

#[derive(Clone, PartialEq)]
pub struct Sorter<T>(pub fn(&T, &T) -> Ordering);
impl<T> Sorter<T> {
    pub const NONE: Sorter<T> = Self(|_, _| Ordering::Equal);
}
impl Feed {
    pub const BY_TITLE: Sorter<Self> = Sorter(|a, b| a.name().cmp(&b.name()));
    pub const BY_LATEST_ITEM: Sorter<Self> =
        Sorter(|a, b| a.state.latest_item_date.cmp(&b.state.latest_item_date));
    pub const BY_TYPE: Sorter<Self> =
        Sorter(|a, b| a.feed_type().to_string().cmp(&b.feed_type().to_string()));
    pub const BY_HITS: Sorter<Self> = Sorter(|a, b| a.state.hits.cmp(&b.state.hits));
    pub const BY_TOT_UNREADS: Sorter<Self> = Sorter(|a, b| a.tot_unread().cmp(&b.tot_unread()));

    pub const BY_TITLE_REV: Sorter<Self> = Sorter(|b, a| a.name().cmp(&b.name()));
    pub const BY_LATEST_ITEM_REV: Sorter<Self> =
        Sorter(|b, a| a.state.latest_item_date.cmp(&b.state.latest_item_date));
    pub const BY_TYPE_REV: Sorter<Self> =
        Sorter(|b, a| a.feed_type().to_string().cmp(&b.feed_type().to_string()));
}
impl Item {
    pub const BY_IS_READ: Sorter<Self> = Sorter(|a, b| a.state.is_read.cmp(&b.state.is_read));
    pub const BY_TITLE: Sorter<Self> = Sorter(|a, b| a.data.title.cmp(&b.data.title));
    pub const BY_POSTED: Sorter<Self> = Sorter(|a, b| a.data.posted.cmp(&b.data.posted));
    pub const BY_TITLE_REV: Sorter<Self> = Sorter(|b, a| a.data.title.cmp(&b.data.title));
    pub const BY_POSTED_REV: Sorter<Self> = Sorter(|b, a| a.data.posted.cmp(&b.data.posted));
}
impl Tag {
    pub const BY_NAME: Sorter<Self> = Sorter(|a, b| a.name.cmp(&b.name));
    pub const BY_COUNT: Sorter<Self> = Sorter(|a, b| a.count.cmp(&b.count));
    pub const BY_NAME_REV: Sorter<Self> = Sorter(|b, a| a.name.cmp(&b.name));
    pub const BY_COUNT_REV: Sorter<Self> = Sorter(|b, a| a.count.cmp(&b.count));
}
impl Link {
    pub const BY_TITLE: Sorter<Self> = Sorter(|a, b| a.0.title.cmp(&b.0.title));
    pub const BY_HREF: Sorter<Self> = Sorter(|a, b| a.0.href.cmp(&b.0.href));
    pub const BY_MIME: Sorter<Self> = Sorter(|a, b| a.0.media_type.cmp(&b.0.media_type));
}

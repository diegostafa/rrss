use std::cmp::Ordering;

use super::models::{Feed, Item, Link, Tag};

#[derive(Clone)]
pub struct Sorter<T>(pub fn(&T, &T) -> Ordering);
impl<T> Sorter<T> {
    pub const NONE: Sorter<T> = Self(|_, _| Ordering::Equal);
}
impl Feed {
    pub const BY_TITLE: Sorter<Self> = Sorter(|a, b| a.name().cmp(&b.name()));
    pub const BY_LATEST_ITEM: Sorter<Self> =
        Sorter(|a, b| a.metrics.latest_item_date.cmp(&b.metrics.latest_item_date));
    pub const BY_TYPE: Sorter<Self> =
        Sorter(|a, b| a.feed_type().to_string().cmp(&b.feed_type().to_string()));
    pub const BY_TITLE_REV: Sorter<Self> = Sorter(|b, a| a.name().cmp(&b.name()));
    pub const BY_LATEST_ITEM_REV: Sorter<Self> =
        Sorter(|b, a| a.metrics.latest_item_date.cmp(&b.metrics.latest_item_date));
    pub const BY_TYPE_REV: Sorter<Self> =
        Sorter(|b, a| a.feed_type().to_string().cmp(&b.feed_type().to_string()));
}
impl Item {
    pub const BY_TITLE: Sorter<Self> = Sorter(|a, b| a.title.cmp(&b.title));
    pub const BY_POSTED: Sorter<Self> = Sorter(|a, b| a.posted.cmp(&b.posted));
    pub const BY_TITLE_REV: Sorter<Self> = Sorter(|b, a| a.title.cmp(&b.title));
    pub const BY_POSTED_REV: Sorter<Self> = Sorter(|b, a| a.posted.cmp(&b.posted));
}
impl Tag {
    pub const BY_NAME: Sorter<Self> = Sorter(|a, b| a.name.cmp(&b.name));
    pub const BY_COUNT: Sorter<Self> = Sorter(|a, b| a.count.cmp(&b.count));
    pub const BY_NAME_REV: Sorter<Self> = Sorter(|b, a| a.name.cmp(&b.name));
    pub const BY_COUNT_REV: Sorter<Self> = Sorter(|b, a| a.count.cmp(&b.count));
}
impl Link {
    pub const BY_TITLE: Sorter<Self> = Sorter(|a, b| a.title.cmp(&b.title));
    pub const BY_HREF: Sorter<Self> = Sorter(|a, b| a.href.cmp(&b.href));
    pub const BY_MIME: Sorter<Self> = Sorter(|a, b| a.mime_type.cmp(&b.mime_type));
}

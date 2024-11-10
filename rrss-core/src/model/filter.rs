use super::models::{Feed, FeedId, Item, ItemId, Tag};

pub trait FilterTest<T> {
    fn test(&self, e: &T) -> bool;
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Filter {
    pub tag_id: Option<String>,
    pub feed_id: Option<FeedId>,
    pub item_id: Option<ItemId>,

    pub unread_feed: Option<bool>,
    pub unread_item: Option<bool>,

    pub feed_contains: Option<String>,
    pub item_contains: Option<String>,
    pub tag_contains: Option<String>,

    pub unfiltered: Option<()>,
}
impl Filter {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tag_id(mut self, tag: String) -> Self {
        self.tag_id = Some(tag);
        self
    }
    pub fn feed_id(mut self, id: FeedId) -> Self {
        self.feed_id = Some(id);
        self
    }
    pub fn item_id(mut self, id: ItemId) -> Self {
        self.item_id = Some(id);
        self
    }
    pub fn unread_feed(mut self) -> Self {
        self.unread_feed = Some(true);
        self
    }
    pub fn read_feed(mut self) -> Self {
        self.unread_feed = Some(false);
        self
    }
    pub fn unread_item(mut self) -> Self {
        self.unread_item = Some(true);
        self
    }
    pub fn read_item(mut self) -> Self {
        self.unread_item = Some(false);
        self
    }
    pub fn feed_contains(mut self, pattern: String) -> Self {
        self.feed_contains = Some(pattern);
        self
    }
    pub fn item_contains(mut self, pattern: String) -> Self {
        self.feed_contains = Some(pattern);
        self
    }
    pub fn tag_contains(mut self, pattern: String) -> Self {
        self.tag_contains = Some(pattern);
        self
    }
    pub fn unfiltered(mut self) -> Self {
        self.unfiltered = Some(());
        self
    }
}
impl FilterTest<Feed> for Filter {
    fn test(&self, e: &Feed) -> bool {
        if let Some(tag) = &self.tag_id {
            return e.conf.tags.contains(tag);
        }
        if let Some(id) = &self.feed_id {
            return e.id() == id;
        }
        if let Some(_) = &self.unread_feed {
            return e.tot_unread() > 0;
        }
        if let Some(p) = &self.feed_contains {
            return e.name().to_ascii_lowercase().contains(&p.to_lowercase());
        }
        true
    }
}
impl FilterTest<Item> for Filter {
    fn test(&self, e: &Item) -> bool {
        if let Some(id) = &self.item_id {
            return e.data.id == *id;
        }
        if let Some(true) = self.unread_item {
            return !e.state.is_read;
        }
        if let Some(false) = self.unread_item {
            return e.state.is_read;
        }
        if let Some(p) = &self.item_contains {
            if let Some(title) = &e.data.title {
                return title.to_lowercase().contains(&p.to_lowercase());
            }
            if let Some(content) = &e.data.content {
                return content.to_lowercase().contains(&p.to_lowercase());
            }
            return false;
        }
        if let Some(_) = self.unfiltered {
            return !e.state.is_filtered;
        }
        true
    }
}
impl FilterTest<Tag> for Filter {
    fn test(&self, e: &Tag) -> bool {
        if let Some(id) = &self.tag_id {
            return e.name == *id;
        }
        if let Some(p) = &self.tag_contains {
            return e.name.to_ascii_lowercase().contains(&p.to_lowercase());
        }
        true
    }
}
impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Filters: - {:?}", self)
    }
}

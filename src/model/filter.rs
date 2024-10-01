use super::models::{Feed, FeedId, Item, ItemId, Tag};

pub trait ApplyFilter<T> {
    fn apply(&self, e: &&T) -> bool;
}

#[derive(Default, Debug, Clone)]
pub struct Filter {
    pub with_tag_name: Option<String>,
    pub with_feed_id: Option<FeedId>,
    pub with_item_id: Option<ItemId>,
    pub unread_feed: Option<()>,
    pub unread_item: Option<bool>,
    pub feed_contains: Option<String>,
    pub item_contains: Option<String>,
    pub tag_contains: Option<String>,
}
impl Filter {
    pub fn with_tag(mut self, tag: String) -> Self {
        self.with_tag_name = Some(tag);
        self
    }
    pub fn with_feed_id(mut self, id: FeedId) -> Self {
        self.with_feed_id = Some(id);
        self
    }
    pub fn with_item_id(mut self, id: ItemId) -> Self {
        self.with_item_id = Some(id);
        self
    }
    pub fn has_unread_items(mut self) -> Self {
        self.unread_feed = Some(());
        self
    }
    pub fn is_item_read(mut self) -> Self {
        self.unread_item = Some(false);
        self
    }
    pub fn is_item_unread(mut self) -> Self {
        self.unread_item = Some(true);
        self
    }
    pub fn with_feed_containing(mut self, pattern: String) -> Self {
        self.feed_contains = Some(pattern);
        self
    }
    pub fn with_item_containing(mut self, pattern: String) -> Self {
        self.feed_contains = Some(pattern);
        self
    }
    pub fn with_tag_containing(mut self, pattern: String) -> Self {
        self.tag_contains = Some(pattern);
        self
    }
}
impl ApplyFilter<Feed> for Filter {
    fn apply(&self, e: &&Feed) -> bool {
        if let Some(tag) = &self.with_tag_name {
            return e.tags().contains(tag);
        }
        if let Some(id) = &self.with_feed_id {
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
impl ApplyFilter<Item> for Filter {
    fn apply(&self, e: &&Item) -> bool {
        if let Some(id) = &self.with_item_id {
            return e.id == *id;
        }
        if let Some(true) = &self.unread_item {
            return !e.is_read;
        }
        if let Some(false) = &self.unread_item {
            return e.is_read;
        }
        if let Some(p) = &self.item_contains {
            if let Some(title) = &e.title {
                return title.to_lowercase().contains(&p.to_lowercase());
            }
            if let Some(content) = &e.content {
                return content.to_lowercase().contains(&p.to_lowercase());
            }
            return false;
        }
        true
    }
}
impl ApplyFilter<Tag> for Filter {
    fn apply(&self, e: &&Tag) -> bool {
        if let Some(id) = &self.with_tag_name {
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

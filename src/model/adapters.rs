use std::fmt::Display;
use std::time::Duration;

use chrono::{DateTime, Utc};
use feed_rs::model::{FeedType, MediaContent, MediaObject};
use serde::{Deserialize, Serialize};

use super::html_to_text;
use super::models::{Item, ItemId, Link};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaContentAdapter {
    pub url: Option<String>,
    pub mime_type: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<Duration>,
    pub file_size: Option<u64>,
}
impl From<MediaContent> for MediaContentAdapter {
    fn from(content: MediaContent) -> Self {
        Self {
            url: content.url.map(|u| u.to_string()),
            mime_type: content.content_type.map(|c| {
                format!(
                    "{}/{}",
                    c.ty().as_str().to_ascii_lowercase(),
                    c.subty().as_str().to_ascii_lowercase()
                )
            }),
            width: content.width,
            height: content.height,
            duration: content.duration,
            file_size: content.size,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaObjectAdapter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub payload: Vec<MediaContentAdapter>,
}
impl From<MediaObject> for MediaObjectAdapter {
    fn from(media: MediaObject) -> Self {
        Self {
            title: media.title.map(|t| t.content),
            description: media.description.map(|d| d.content),
            payload: media
                .content
                .into_iter()
                .map(MediaContentAdapter::from)
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum FeedTypeAdapter {
    #[default]
    Unknown,
    Atom,
    JSON,
    RSS0,
    RSS1,
    RSS2,
}
impl From<FeedType> for FeedTypeAdapter {
    fn from(value: FeedType) -> Self {
        match value {
            FeedType::Atom => FeedTypeAdapter::Atom,
            FeedType::JSON => FeedTypeAdapter::JSON,
            FeedType::RSS0 => FeedTypeAdapter::RSS0,
            FeedType::RSS1 => FeedTypeAdapter::RSS1,
            FeedType::RSS2 => FeedTypeAdapter::RSS2,
        }
    }
}
impl Display for FeedTypeAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedTypeAdapter::Unknown => write!(f, "Unknown"),
            FeedTypeAdapter::Atom => write!(f, "Atom"),
            FeedTypeAdapter::JSON => write!(f, "JSON"),
            FeedTypeAdapter::RSS0 => write!(f, "RSS0"),
            FeedTypeAdapter::RSS1 => write!(f, "RSS1"),
            FeedTypeAdapter::RSS2 => write!(f, "RSS2"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkAdapter {
    pub href: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
}
impl From<feed_rs::model::Link> for LinkAdapter {
    fn from(link: feed_rs::model::Link) -> Self {
        Self {
            href: link.href,
            title: link.title,
            mime_type: link.media_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedAdapter {
    pub feed_type: FeedTypeAdapter,
    pub title: String,
    pub items: Vec<Item>,
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub links: Vec<Link>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub language: Option<String>,
}
impl From<feed_rs::model::Feed> for FeedAdapter {
    fn from(feed: feed_rs::model::Feed) -> Self {
        Self {
            feed_type: FeedTypeAdapter::from(feed.feed_type),
            title: feed.title.map(|t| t.content).unwrap_or_default(),
            items: feed.entries.into_iter().map(Item::from).collect::<Vec<_>>(),
            published: feed.published,
            updated: feed.updated,
            links: feed.links.into_iter().map(Link::from).collect(),
            authors: feed.authors.into_iter().map(|a| a.name).collect(),
            description: feed.description.map(|d| d.content),
            categories: feed.categories.into_iter().map(|c| c.term).collect(),
            language: feed.language,
        }
    }
}

impl From<feed_rs::model::Entry> for Item {
    fn from(item: feed_rs::model::Entry) -> Self {
        Self {
            is_read: false,
            is_filtered: false,
            id: ItemId(item.id),
            title: item.title.map(|t| t.content),
            content: item
                .content
                .and_then(|s| s.body)
                .as_deref()
                .map(html_to_text),
            summary: item.summary.map(|s| html_to_text(&s.content)),
            posted: item.published.or(item.updated),
            links: item.links.into_iter().map(Link::from).collect(),
            media: item
                .media
                .into_iter()
                .map(MediaObjectAdapter::from)
                .collect(),
        }
    }
}

impl From<feed_rs::model::Link> for Link {
    fn from(link: feed_rs::model::Link) -> Self {
        Self {
            href: link.href,
            title: link.title,
            mime_type: link.media_type,
        }
    }
}

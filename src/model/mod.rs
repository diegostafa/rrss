use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use html2text::config;

use crate::globals::CONFIG;

pub mod adapters;
pub mod filter;
pub mod models;
pub mod sorter;

pub fn format_date(date: DateTime<Utc>) -> String {
    let delta_days = (Utc::now() - date).num_days();
    match delta_days {
        0 => HumanTime::from(date).to_text_en(Accuracy::Rough, Tense::Past),
        _ if delta_days < CONFIG.max_days_until_old as i64 => {
            format!("{}, {}", HumanTime::from(date), date.format("%a, %H:%M"))
        }
        _ => date.format(CONFIG.theme.date_format.as_str()).to_string(),
    }
}

pub fn html_to_text(html: &str) -> String {
    config::plain()
        .raw_mode(true)
        .no_table_borders()
        .string_from_read(html.as_bytes(), 1000)
        .unwrap()
        .trim()
        .to_string()
}

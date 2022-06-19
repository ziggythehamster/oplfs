use chrono::{Date, DateTime};
use chrono::offset::Utc;

use crate::schema::discs;

use super::Format;
use super::Media;
use super::Platform;
use super::VideoMode;

/// A disc in the database, which will be made accessible via the filesystem
#[derive(Debug, Identifiable, Queryable)]
pub struct Disc<'a> {
    pub id: Option<u32>,
    pub title_id: &'a str,
    pub path: &'a str,
    pub media: Option<Media>,
    pub format: Option<Format>,
    pub platform: Option<Platform>,
    pub size: u64,
    pub title: &'a str,
    pub description: &'a str,
    pub developer: &'a str,
    pub genre: &'a str,
    pub video_mode: Option<VideoMode>,
    pub release: Option<Date<Utc>>,
    pub players: Option<u8>,
    pub rating_stars: Option<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

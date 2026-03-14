use serde::{Deserialize, Serialize};
use specta::Type;
use yaserde::{YaDeserialize, YaSerialize};

use super::Comic;

/// https://anansi-project.github.io/comicinfo/schemas/v2.0/ComicInfo.xsd
#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type, YaSerialize, YaDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfo {
    /// Chapter title
    #[yaserde(rename = "Title")]
    pub title: String,
    /// Series title
    #[yaserde(rename = "Series")]
    pub series: String,
    /// Is this a manga? (Yes/No/YesAndRightToLeft)
    #[yaserde(rename = "Manga")]
    pub manga: String,
    #[yaserde(rename = "Writer")]
    pub writer: String,
    #[yaserde(rename = "Publisher")]
    pub publisher: String,
    #[yaserde(rename = "Genre")]
    pub genre: String,
    #[yaserde(rename = "Tags")]
    pub tags: String,
    /// Normal chapter number
    #[yaserde(rename = "Number")]
    pub number: Option<String>,
    /// Volume number
    #[yaserde(rename = "Volume")]
    pub volume: Option<String>,
    /// if the value is `Special`, the chapter will be treated as a special issue by Kavita
    #[yaserde(rename = "Format")]
    pub format: Option<String>,
    /// The number of pages in this chapter
    #[yaserde(rename = "PageCount")]
    pub page_count: i64,
    /// Total number of chapters
    /// - `0` => Ongoing
    /// - `Non-zero` and consistent with `Number` or `Volume` => Completed
    /// - `Other non-zero values` => Ended
    #[yaserde(rename = "Count")]
    pub count: i64,
    /// Language of the comic (ISO 639-1 code, e.g., "en", "ja", "zh")
    #[yaserde(rename = "LanguageISO")]
    pub language_iso: Option<String>,
}

impl From<Comic> for ComicInfo {
    fn from(comic: Comic) -> Self {
        // Convert hitomi language to ISO 639-1 code
        let language_iso = match comic.language.as_str() {
            "english" => Some("en".to_string()),
            "japanese" => Some("ja".to_string()),
            "chinese" => Some("zh".to_string()),
            "korean" => Some("ko".to_string()),
            "spanish" => Some("es".to_string()),
            "french" => Some("fr".to_string()),
            "german" => Some("de".to_string()),
            "italian" => Some("it".to_string()),
            "portuguese" => Some("pt".to_string()),
            "russian" => Some("ru".to_string()),
            "thai" => Some("th".to_string()),
            "vietnamese" => Some("vi".to_string()),
            "polish" => Some("pl".to_string()),
            "indonesian" => Some("id".to_string()),
            _ => Some(comic.language), // fallback to original if unknown
        };

        ComicInfo {
            title: comic.title.clone(),
            series: comic.title,
            manga: "Yes".to_string(),
            writer: comic.artists.join(", "),
            publisher: "Hitomi".to_string(),
            genre: comic.type_field,
            tags: comic
                .tags
                .into_iter()
                .map(|tag| tag.tag)
                .collect::<Vec<String>>()
                .join(", "),
            number: Some("1".to_string()),
            volume: None,
            format: Some("Special".to_string()),
            #[allow(clippy::cast_possible_wrap)]
            page_count: comic.files.len() as i64,
            count: 1,
            language_iso,
        }
    }
}

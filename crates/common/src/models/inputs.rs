use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::time::TimeRangePreset;

#[derive(Serialize, Deserialize, Debug)]
pub struct EventInput {
    pub timestamp: Option<NaiveDateTime>,
    pub duration: Option<i64>,
    pub activity_type: String,
    pub app_name: String,
    pub entity_name: String,
    pub entity_type: String,
    pub project_name: String,
    pub project_path: String,
    pub branch_name: String,
    pub language_name: String,
    pub end_timestamp: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatInput {
    pub timestamp: Option<NaiveDateTime>,
    pub project_name: String,
    pub project_path: String,
    pub entity_name: String,
    pub entity_type: String,
    pub branch_name: String,
    pub language_name: Option<String>,
    pub app_name: String,
    pub is_write: bool,
    pub lines: Option<i64>,
    pub cursorpos: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AFKEventInput {
    pub afk_start: NaiveDateTime,
    pub afk_end: Option<NaiveDateTime>,
    pub duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Group {
    App,
    Project,
    Language,
    Branch,
    Category,
    Entity,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct SummaryQueryInput {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    #[specta(optional)]
    pub app_names: Option<Vec<String>>,
    #[specta(optional)]
    pub project_names: Option<Vec<String>>,
    #[specta(optional)]
    pub activity_types: Option<Vec<String>>,
    #[specta(optional)]
    pub entity_names: Option<Vec<String>>,
    #[specta(optional)]
    pub branch_names: Option<Vec<String>>,
    #[specta(optional)]
    pub language_names: Option<Vec<String>>,
    pub include_afk: bool,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct BucketedSummaryInput {
    pub preset: TimeRangePreset,
    #[specta(optional)]
    pub app_names: Option<Vec<String>>,
    #[specta(optional)]
    pub project_names: Option<Vec<String>>,
    #[specta(optional)]
    pub entity_names: Option<Vec<String>>,
    #[specta(optional)]
    pub activity_types: Option<Vec<String>>,
    #[specta(optional)]
    pub branch_names: Option<Vec<String>>,
    #[specta(optional)]
    pub language_names: Option<Vec<String>>,
    #[specta(optional)]
    pub group_by: Option<Group>,
    pub include_afk: bool,
}

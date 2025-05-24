use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub user_id: String,
    pub full_name: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub gender: Option<Gender>,
    pub birth_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}
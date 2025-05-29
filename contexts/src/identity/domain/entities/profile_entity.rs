use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileEntity {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

impl From<drivers::database::schemas::profile::Profile> for ProfileEntity {
    fn from(profile: drivers::database::schemas::profile::Profile) -> Self {
        Self {
            id: profile.id,
            user_id: profile.user_id,
            full_name: profile.full_name,
            nickname: profile.nickname,
            avatar_url: profile.avatar_url,
            gender: profile.gender.map(|g| match g {
                drivers::database::schemas::profile::Gender::Male => Gender::Male,
                drivers::database::schemas::profile::Gender::Female => Gender::Female,
                drivers::database::schemas::profile::Gender::Other => Gender::Other,
                drivers::database::schemas::profile::Gender::PreferNotToSay => Gender::PreferNotToSay,
            }),
            birth_date: profile.birth_date,
            phone_number: profile.phone_number,
            bio: profile.bio,
            location: profile.location,
            website: profile.website,
            social_links: profile.social_links,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}

impl From<ProfileEntity> for drivers::database::schemas::profile::Profile {
    fn from(entity: ProfileEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            full_name: entity.full_name,
            nickname: entity.nickname,
            avatar_url: entity.avatar_url,
            gender: entity.gender.map(|g| match g {
                Gender::Male => drivers::database::schemas::profile::Gender::Male,
                Gender::Female => drivers::database::schemas::profile::Gender::Female,
                Gender::Other => drivers::database::schemas::profile::Gender::Other,
                Gender::PreferNotToSay => drivers::database::schemas::profile::Gender::PreferNotToSay,
            }),
            birth_date: entity.birth_date,
            phone_number: entity.phone_number,
            bio: entity.bio,
            location: entity.location,
            website: entity.website,
            social_links: entity.social_links,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
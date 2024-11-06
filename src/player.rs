use std::collections::HashMap;

use serde::{de, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
	pub name: String,
	pub prefix: String,
	pub rank: String,
	pub rank_stars: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatList {
	pub total: u32,
	pub list: HashMap<String, u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PvpStats {
	pub kills: u32,
	pub deaths: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalData {
	pub wars: u32,
	pub total_level: u32,
	pub killed_mobs: u32,
	pub chests_found: u32,
	pub completed_quests: u32,

	pub dungeons: StatList,
	pub raids: StatList,

	pub pvp: PvpStats,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
	pub username: String,
	pub online: bool,
	pub server: String,
	pub active_character: Option<String>,
	pub uuid: String,
	pub rank: String,
	pub rank_badge: String,
	// pub legacyRankColor { main: String, sub: String }
	pub shortened_rank: Option<String>,
	pub support_rank: String,
	#[serde(deserialize_with = "null_boolean")]
	pub veteran: bool,
	pub first_join: String,
	pub last_join: String,
	pub playtime: f64,
	pub guild: Option<Guild>,
	pub global_data: GlobalData,
	pub forum_link: Option<u32>,
	pub ranking: HashMap<String, u32>,
	pub previous_ranking: HashMap<String, u32>,
	pub public_profile: bool,
}

fn null_boolean<'d, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: de::Deserializer<'d>,
{
	Ok(Option::deserialize(deserializer)?.unwrap_or(false))
}

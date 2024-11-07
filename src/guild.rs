use std::{collections::HashMap, ops::Deref};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberData {
	pub online: bool,
	pub server: Option<String>,
	pub contributed: u64,
	pub guild_rank: Option<u32>,
	pub joined: String, // Replace (Duration)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberList {
	pub total: u32,
	pub owner: HashMap<String, MemberData>, // Multiple???
	pub chief: HashMap<String, MemberData>,
	pub strategist: HashMap<String, MemberData>,
	pub captain: HashMap<String, MemberData>,
	pub recruiter: HashMap<String, MemberData>,
	pub recruit: HashMap<String, MemberData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildResponse {
	pub uuid: String,
	pub name: String,
	pub prefix: String,
	pub level: u32,
	pub xp_percent: u8,
	pub territories: u32,
	pub wars: u32,
	pub created: String, // Replace (Duration)
	pub members: MemberList,
	pub online: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildData {
	pub uuid: String,
	pub prefix: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildList(HashMap<String, GuildData>);

impl Deref for GuildList {
	type Target = HashMap<String, GuildData>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildMulti {
	pub name: String,
	pub prefix: String,
}

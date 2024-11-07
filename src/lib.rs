use guild::{GuildList, GuildMulti, GuildResponse};
use player::{PlayerMulti, PlayerResponse};
use reqwest::{Client, Response, StatusCode, Url};

pub mod guild;
pub mod player;

mod error;
mod multi_selector;
mod util;

pub use error::{Error, Result};
pub use multi_selector::MaybeMulti;
use serde::de::DeserializeOwned;

const WYNNCRAFT_API: &str = "https://api.wynncraft.com/v3/";

pub struct WynnApi {
	client: Client,

	player_url: Url,
	guild_url: Url,
	guild_prefix_url: Url,
	guild_list_url: Url,
}

impl WynnApi {
	pub fn new() -> Self {
		let client = Client::new();

		let wynn_url = Url::parse(WYNNCRAFT_API).unwrap();
		let player_url = wynn_url.join("player/").unwrap();
		let guild_url = wynn_url.join("guild/").unwrap();
		let guild_prefix_url = guild_url.join("prefix/").unwrap();
		let guild_list_url = guild_url.join("list/guild").unwrap();

		Self {
			client,
			player_url,
			guild_url,
			guild_prefix_url,
			guild_list_url,
		}
	}

	async fn get(&self, url: Url) -> crate::Result<Response> {
		self.client
			.get(url)
			.send()
			.await
			.map_err(crate::Error::request)
	}

	fn decode_json<R>(json: &str) -> crate::Result<R>
	where
		R: DeserializeOwned,
	{
		serde_json::from_str(json).map_err(crate::Error::parse)
	}

	pub async fn query_user(&self, name: &str) -> crate::Result<MaybeMulti<PlayerMulti, PlayerResponse>> {
		let url = self.player_url.join(name).unwrap();

		let res = self.get(url).await?;

		let status = res.status();
		let json = res.text().await.map_err(crate::Error::request)?;

		if status == StatusCode::MULTIPLE_CHOICES {
			Ok(MaybeMulti::Multi(Self::decode_json(&json)?))
		} else {
			Ok(MaybeMulti::Single(Self::decode_json(&json)?))
		}
	}

	pub async fn query_guild_name(&self, name: &str) -> crate::Result<MaybeMulti<GuildMulti, GuildResponse>> {
		let url = self.guild_url.join(name).unwrap();

		let res = self.get(url).await?;

		let status = res.status();
		let json = res.text().await.map_err(crate::Error::request)?;

		if status == StatusCode::MULTIPLE_CHOICES {
			Ok(MaybeMulti::Multi(Self::decode_json(&json)?))
		} else {
			Ok(MaybeMulti::Single(Self::decode_json(&json)?))
		}
	}

	pub async fn query_guild_prefix(&self, prefix: &str) -> crate::Result<GuildResponse> {
		let url = self.guild_prefix_url.join(prefix).unwrap();

		let res = self.get(url).await?;
		let json = res.text().await.map_err(crate::Error::request)?;

		Self::decode_json(&json)
	}

	pub async fn query_guild_list(&self) -> crate::Result<GuildList> {
		let res = self.get(self.guild_list_url.clone()).await?;
		let json = res.text().await.map_err(crate::Error::request)?;

		Self::decode_json(&json)
	}
}

impl Default for WynnApi {
	fn default() -> Self {
		Self::new()
	}
}

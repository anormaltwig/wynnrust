use error::Kind;
use guild::{GuildList, GuildResponse};
use player::PlayerResponse;
use reqwest::{Client, Url};

pub mod guild;
pub mod player;

mod error;

pub use error::{Error, Result};
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
		let guild_list_url= guild_url.join("list/guild").unwrap();

		Self {
			client,
			player_url,
			guild_url,
			guild_prefix_url,
			guild_list_url,
		}
	}

	async fn json_req<R>(&self, url: Url) -> crate::Result<R>
	where
		R: DeserializeOwned,
	{
		let json = match self.client.get(url).send().await {
			Ok(res) => match res.text().await {
				Ok(text) => text,
				Err(err) => {
					return Err(crate::Error::new(Kind::Parse, Box::new(err)));
				}
			},
			Err(err) => {
				return Err(crate::Error::new(Kind::Request, Box::new(err)));
			}
		};

		// Easy pretty print for debug.
		// println!("{}", serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&json).unwrap()).unwrap());

		let response: R = match serde_json::from_str(&json) {
			Ok(res) => res,
			Err(err) => {
				return Err(crate::Error::new(Kind::Parse, Box::new(err)));
			}
		};

		Ok(response)
	}

	pub async fn query_user(&self, name: &str) -> crate::Result<PlayerResponse> {
		let url = self.player_url.join(name).unwrap();
		self.json_req(url).await
	}

	pub async fn query_guild_name(&self, name: &str) -> crate::Result<GuildResponse> {
		let url = self.guild_url.join(name).unwrap();
		self.json_req(url).await
	}

	pub async fn query_guild_prefix(&self, prefix: &str) -> crate::Result<GuildResponse> {
		let url = self.guild_prefix_url.join(prefix).unwrap();
		self.json_req(url).await
	}

	pub async fn query_guild_list(&self) -> crate::Result<GuildList> {
		self.json_req(self.guild_list_url.clone()).await
	}
}

impl Default for WynnApi {
	fn default() -> Self {
		Self::new()
	}
}

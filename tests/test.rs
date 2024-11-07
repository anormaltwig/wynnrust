use wynnrust::WynnApi;

#[tokio::test]
async fn player_query() {
	let api = WynnApi::new();

	dbg!(api.query_user("ANormalTwig").await.unwrap());
}

#[tokio::test]
async fn guild_query() {
	let api = WynnApi::new();

	api.query_guild_name("Sequoia").await.unwrap();
	api.query_guild_prefix("SEQ").await.unwrap();

	api.query_guild_list().await.unwrap();
}

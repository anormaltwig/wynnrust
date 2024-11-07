use serde::{de, Deserialize};

pub(crate) fn deserialize_null_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: de::Deserializer<'de>,
{
	Ok(Option::deserialize(deserializer)?.unwrap_or(false))
}

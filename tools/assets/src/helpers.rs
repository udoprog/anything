use rational::Rational;
use serde::de;
use serde::Deserialize;

pub(crate) fn deserialize_number<'de, D>(deserializer: D) -> Result<Rational, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n = serde_json::Number::deserialize(deserializer)?.to_string();
    str::parse::<Rational>(&n).map_err(<D::Error as de::Error>::custom)
}

pub(crate) fn deserialize_option_number<'de, D>(
    deserializer: D,
) -> Result<Option<Rational>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n = Option::<serde_json::Number>::deserialize(deserializer)?;

    let n = match n {
        Some(n) => n.to_string(),
        None => return Ok(None),
    };

    Ok(Some(
        str::parse::<Rational>(&n).map_err(<D::Error as de::Error>::custom)?,
    ))
}

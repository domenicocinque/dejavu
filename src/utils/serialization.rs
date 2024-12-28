use image_hasher::{ImageHash, InvalidBytesError};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serializer};

pub fn hash_to_base64<S>(hash: &ImageHash, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&hash.to_base64())
}

pub fn hash_from_base64<'de, D>(deserializer: D) -> Result<ImageHash, D::Error>
where
    D: Deserializer<'de>,
{
    let base64_str = String::deserialize(deserializer)?;
    ImageHash::from_base64(&base64_str).map_err(|err: InvalidBytesError| {
        D::Error::custom(format!("Failed to parse hash from base64: {:?}", err))
    })
}

use std::str::FromStr;

use qsimplify::{GateType, GateTypeError};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[derive(Debug, Clone)]
pub(crate) struct ReadableGateType(pub(crate) GateType);

impl From<ReadableGateType> for String {
    fn from(value: ReadableGateType) -> Self {
        value.0.to_string()
    }
}

impl TryFrom<String> for ReadableGateType {
    type Error = GateTypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(GateType::from_str(&value)?))
    }
}

impl Serialize for ReadableGateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for ReadableGateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        GateType::from_str(&value).map(Self).map_err(Error::custom)
    }
}

use qsimplify::GateType;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[derive(Debug, Clone)]
pub(crate) struct BinaryGateType(pub(crate) GateType);

impl Serialize for BinaryGateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0.into())
    }
}

impl<'de> Deserialize<'de> for BinaryGateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;

        GateType::try_from(value).map(Self).map_err(Error::custom)
    }
}

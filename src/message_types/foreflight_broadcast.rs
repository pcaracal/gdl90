use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct Port {
    pub port: u16,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
pub struct ForeFlightBroadcast {
    #[serde(rename = "App")]
    pub app: String,
    #[serde(rename = "GDL90")]
    pub gdl90: Port,
}

#[allow(clippy::missing_errors_doc)]
impl ForeFlightBroadcast {
    pub fn from_json(data: impl AsRef<[u8]>) -> serde_json::Result<Self> {
        serde_json::from_slice(data.as_ref())
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_foreflight_broadcast() {
        let json_data = r#"{"App":"ForeFlight","GDL90":{"port":4000}}"#;
        let broadcast = ForeFlightBroadcast::from_json(json_data).unwrap();
        assert_eq!(broadcast.app, "ForeFlight");
        assert_eq!(broadcast.gdl90.port, 4000);
        let json_output = broadcast.to_json().unwrap();
        let expected_json = r#"{"App":"ForeFlight","GDL90":{"port":4000}}"#;
        assert_eq!(json_output, expected_json);
    }
}

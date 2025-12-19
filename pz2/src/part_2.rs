use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum RequestType {
    Success,
    Failure,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    duration: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    duration: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: String,
    is_private: bool,
    settings: u32,
    shard_url: String,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DebugInfo {
    duration: String,
    at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: DebugInfo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_toml() {
        let json_data = r#"
        {
          "type": "success",
          "stream": {
            "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
            "is_private": false,
            "settings": 45345,
            "shard_url": "https://n3.example.com/sapi",
            "public_tariff": {
              "id": 1,
              "price": 100,
              "duration": "1h",
              "description": "test public tariff"
            },
            "private_tariff": {
              "client_price": 250,
              "duration": "1m",
              "description": "test private tariff"
            }
          },
          "gifts": [{
            "id": 1,
            "price": 2,
            "description": "Gift 1"
          }, {
            "id": 2,
            "price": 3,
            "description": "Gift 2"
          }],
          "debug": {
            "duration": "234ms",
            "at": "2019-06-28T08:35:46+00:00"
          }
        }
        "#;

        let request: Request = serde_json::from_str(json_data).expect("Failed to parse JSON");
        
        assert_eq!(request.request_type, RequestType::Success);
        assert_eq!(request.stream.settings, 45345);
        assert_eq!(request.stream.public_tariff.price, 100);
        assert_eq!(request.stream.private_tariff.client_price, 250);

        let toml_output = toml::to_string(&request).expect("Failed to serialize to TOML");
        println!("{}", toml_output);
        
        assert!(toml_output.contains("shard_url"));
        assert!(toml_output.contains("test public tariff"));
    }
}

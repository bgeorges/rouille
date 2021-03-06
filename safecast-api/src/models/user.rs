/*
 * Safecast API
 *
 * Safecast API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "measurements_count", skip_serializing_if = "Option::is_none")]
    pub measurements_count: Option<i64>,
}

impl User {
    pub fn new(id: i64, name: String) -> User {
        User {
            id,
            name,
            measurements_count: None,
        }
    }
}



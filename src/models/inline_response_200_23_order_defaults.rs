/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineResponse20023OrderDefaults {
  /// orderType
  #[serde(rename = "string")]
  string: Option<Vec<::models::InlineResponse20023String>>
}

impl InlineResponse20023OrderDefaults {
  pub fn new() -> InlineResponse20023OrderDefaults {
    InlineResponse20023OrderDefaults {
      string: None
    }
  }

  pub fn set_string(&mut self, string: Vec<::models::InlineResponse20023String>) {
    self.string = Some(string);
  }

  pub fn with_string(mut self, string: Vec<::models::InlineResponse20023String>) -> InlineResponse20023OrderDefaults {
    self.string = Some(string);
    self
  }

  pub fn string(&self) -> Option<&Vec<::models::InlineResponse20023String>> {
    self.string.as_ref()
  }

  pub fn reset_string(&mut self) {
    self.string = None;
  }

}




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
pub struct Auth {
  #[serde(rename = "response")]
  response: Option<String>
}

impl Auth {
  pub fn new() -> Auth {
    Auth {
      response: None
    }
  }

  pub fn set_response(&mut self, response: String) {
    self.response = Some(response);
  }

  pub fn with_response(mut self, response: String) -> Auth {
    self.response = Some(response);
    self
  }

  pub fn response(&self) -> Option<&String> {
    self.response.as_ref()
  }

  pub fn reset_response(&mut self) {
    self.response = None;
  }

}




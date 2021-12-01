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
pub struct InlineResponse20036 {
  #[serde(rename = "symbol")]
  symbol: Option<::models::Futures>
}

impl InlineResponse20036 {
  pub fn new() -> InlineResponse20036 {
    InlineResponse20036 {
      symbol: None
    }
  }

  pub fn set_symbol(&mut self, symbol: ::models::Futures) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: ::models::Futures) -> InlineResponse20036 {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&::models::Futures> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

}




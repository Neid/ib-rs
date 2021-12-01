/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecdefInnerIncrementRules : Price increment value contract trades.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecdefInnerIncrementRules {
  /// The minimum increment value for contract price.
  #[serde(rename = "increment")]
  increment: Option<f32>,
  /// The minimum contract price on the market that supports the specified increment.
  #[serde(rename = "lowerEdge")]
  lower_edge: Option<f32>
}

impl SecdefInnerIncrementRules {
  /// Price increment value contract trades.
  pub fn new() -> SecdefInnerIncrementRules {
    SecdefInnerIncrementRules {
      increment: None,
      lower_edge: None
    }
  }

  pub fn set_increment(&mut self, increment: f32) {
    self.increment = Some(increment);
  }

  pub fn with_increment(mut self, increment: f32) -> SecdefInnerIncrementRules {
    self.increment = Some(increment);
    self
  }

  pub fn increment(&self) -> Option<&f32> {
    self.increment.as_ref()
  }

  pub fn reset_increment(&mut self) {
    self.increment = None;
  }

  pub fn set_lower_edge(&mut self, lower_edge: f32) {
    self.lower_edge = Some(lower_edge);
  }

  pub fn with_lower_edge(mut self, lower_edge: f32) -> SecdefInnerIncrementRules {
    self.lower_edge = Some(lower_edge);
    self
  }

  pub fn lower_edge(&self) -> Option<&f32> {
    self.lower_edge.as_ref()
  }

  pub fn reset_lower_edge(&mut self) {
    self.lower_edge = None;
  }

}




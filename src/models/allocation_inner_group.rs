/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AllocationInnerGroup : portfolio allocation by group

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AllocationInnerGroup {
  #[serde(rename = "long")]
  long: Option<::models::AllocationInnerGroupLong>,
  #[serde(rename = "short")]
  short: Option<::models::AllocationInnerGroupShort>
}

impl AllocationInnerGroup {
  /// portfolio allocation by group
  pub fn new() -> AllocationInnerGroup {
    AllocationInnerGroup {
      long: None,
      short: None
    }
  }

  pub fn set_long(&mut self, long: ::models::AllocationInnerGroupLong) {
    self.long = Some(long);
  }

  pub fn with_long(mut self, long: ::models::AllocationInnerGroupLong) -> AllocationInnerGroup {
    self.long = Some(long);
    self
  }

  pub fn long(&self) -> Option<&::models::AllocationInnerGroupLong> {
    self.long.as_ref()
  }

  pub fn reset_long(&mut self) {
    self.long = None;
  }

  pub fn set_short(&mut self, short: ::models::AllocationInnerGroupShort) {
    self.short = Some(short);
  }

  pub fn with_short(mut self, short: ::models::AllocationInnerGroupShort) -> AllocationInnerGroup {
    self.short = Some(short);
    self
  }

  pub fn short(&self) -> Option<&::models::AllocationInnerGroupShort> {
    self.short.as_ref()
  }

  pub fn reset_short(&mut self) {
    self.short = None;
  }

}




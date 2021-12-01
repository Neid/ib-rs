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
pub struct ScannerResult {
  #[serde(rename = "Contracts")]
  contracts: Option<::models::ScannerresultContracts>,
  #[serde(rename = "id")]
  id: Option<f32>,
  #[serde(rename = "offset")]
  offset: Option<i32>,
  #[serde(rename = "position")]
  position: Option<String>,
  #[serde(rename = "scanTime")]
  scan_time: Option<String>,
  #[serde(rename = "size")]
  size: Option<i32>,
  #[serde(rename = "total")]
  total: Option<i32>
}

impl ScannerResult {
  pub fn new() -> ScannerResult {
    ScannerResult {
      contracts: None,
      id: None,
      offset: None,
      position: None,
      scan_time: None,
      size: None,
      total: None
    }
  }

  pub fn set_contracts(&mut self, contracts: ::models::ScannerresultContracts) {
    self.contracts = Some(contracts);
  }

  pub fn with_contracts(mut self, contracts: ::models::ScannerresultContracts) -> ScannerResult {
    self.contracts = Some(contracts);
    self
  }

  pub fn contracts(&self) -> Option<&::models::ScannerresultContracts> {
    self.contracts.as_ref()
  }

  pub fn reset_contracts(&mut self) {
    self.contracts = None;
  }

  pub fn set_id(&mut self, id: f32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: f32) -> ScannerResult {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&f32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_offset(&mut self, offset: i32) {
    self.offset = Some(offset);
  }

  pub fn with_offset(mut self, offset: i32) -> ScannerResult {
    self.offset = Some(offset);
    self
  }

  pub fn offset(&self) -> Option<&i32> {
    self.offset.as_ref()
  }

  pub fn reset_offset(&mut self) {
    self.offset = None;
  }

  pub fn set_position(&mut self, position: String) {
    self.position = Some(position);
  }

  pub fn with_position(mut self, position: String) -> ScannerResult {
    self.position = Some(position);
    self
  }

  pub fn position(&self) -> Option<&String> {
    self.position.as_ref()
  }

  pub fn reset_position(&mut self) {
    self.position = None;
  }

  pub fn set_scan_time(&mut self, scan_time: String) {
    self.scan_time = Some(scan_time);
  }

  pub fn with_scan_time(mut self, scan_time: String) -> ScannerResult {
    self.scan_time = Some(scan_time);
    self
  }

  pub fn scan_time(&self) -> Option<&String> {
    self.scan_time.as_ref()
  }

  pub fn reset_scan_time(&mut self) {
    self.scan_time = None;
  }

  pub fn set_size(&mut self, size: i32) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: i32) -> ScannerResult {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&i32> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_total(&mut self, total: i32) {
    self.total = Some(total);
  }

  pub fn with_total(mut self, total: i32) -> ScannerResult {
    self.total = Some(total);
    self
  }

  pub fn total(&self) -> Option<&i32> {
    self.total.as_ref()
  }

  pub fn reset_total(&mut self) {
    self.total = None;
  }

}




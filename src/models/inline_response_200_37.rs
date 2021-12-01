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
pub struct InlineResponse20037 {
  /// Exchange parameter id
  #[serde(rename = "id")]
  id: Option<String>,
  /// Always contains at least one 'tradingTime'  and zero or more 'sessionTime' tags
  #[serde(rename = "schedules")]
  schedules: Option<Vec<::models::InlineResponse20037Schedules>>,
  /// Reference on a trade venue of given exchange parameter
  #[serde(rename = "tradeVenueId")]
  trade_venue_id: Option<String>
}

impl InlineResponse20037 {
  pub fn new() -> InlineResponse20037 {
    InlineResponse20037 {
      id: None,
      schedules: None,
      trade_venue_id: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> InlineResponse20037 {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_schedules(&mut self, schedules: Vec<::models::InlineResponse20037Schedules>) {
    self.schedules = Some(schedules);
  }

  pub fn with_schedules(mut self, schedules: Vec<::models::InlineResponse20037Schedules>) -> InlineResponse20037 {
    self.schedules = Some(schedules);
    self
  }

  pub fn schedules(&self) -> Option<&Vec<::models::InlineResponse20037Schedules>> {
    self.schedules.as_ref()
  }

  pub fn reset_schedules(&mut self) {
    self.schedules = None;
  }

  pub fn set_trade_venue_id(&mut self, trade_venue_id: String) {
    self.trade_venue_id = Some(trade_venue_id);
  }

  pub fn with_trade_venue_id(mut self, trade_venue_id: String) -> InlineResponse20037 {
    self.trade_venue_id = Some(trade_venue_id);
    self
  }

  pub fn trade_venue_id(&self) -> Option<&String> {
    self.trade_venue_id.as_ref()
  }

  pub fn reset_trade_venue_id(&mut self) {
    self.trade_venue_id = None;
  }

}




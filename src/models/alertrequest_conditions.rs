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
pub struct AlertrequestConditions {
  /// conid and exchange. Format supports conid or conid@exchange
  #[serde(rename = "conidex")]
  conidex: Option<String>,
  /// \"a\" means \"AND\", \"o\" means \"OR\", \"n\" means \"END\", the last one condition in the condition array should \"n\" 
  #[serde(rename = "logicBind")]
  logic_bind: Option<String>,
  /// optional, operator for the current condition, can be >= or <=
  #[serde(rename = "operator")]
  operator: Option<String>,
  /// only needed for some MTA alert condition
  #[serde(rename = "timeZone")]
  time_zone: Option<String>,
  /// optional, only some type of conditions have triggerMethod
  #[serde(rename = "triggerMethod")]
  trigger_method: Option<String>,
  /// Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN& 
  #[serde(rename = "type")]
  _type: Option<i32>,
  /// can not be empty, can pass default value \"*\"
  #[serde(rename = "value")]
  value: Option<String>
}

impl AlertrequestConditions {
  pub fn new() -> AlertrequestConditions {
    AlertrequestConditions {
      conidex: None,
      logic_bind: None,
      operator: None,
      time_zone: None,
      trigger_method: None,
      _type: None,
      value: None
    }
  }

  pub fn set_conidex(&mut self, conidex: String) {
    self.conidex = Some(conidex);
  }

  pub fn with_conidex(mut self, conidex: String) -> AlertrequestConditions {
    self.conidex = Some(conidex);
    self
  }

  pub fn conidex(&self) -> Option<&String> {
    self.conidex.as_ref()
  }

  pub fn reset_conidex(&mut self) {
    self.conidex = None;
  }

  pub fn set_logic_bind(&mut self, logic_bind: String) {
    self.logic_bind = Some(logic_bind);
  }

  pub fn with_logic_bind(mut self, logic_bind: String) -> AlertrequestConditions {
    self.logic_bind = Some(logic_bind);
    self
  }

  pub fn logic_bind(&self) -> Option<&String> {
    self.logic_bind.as_ref()
  }

  pub fn reset_logic_bind(&mut self) {
    self.logic_bind = None;
  }

  pub fn set_operator(&mut self, operator: String) {
    self.operator = Some(operator);
  }

  pub fn with_operator(mut self, operator: String) -> AlertrequestConditions {
    self.operator = Some(operator);
    self
  }

  pub fn operator(&self) -> Option<&String> {
    self.operator.as_ref()
  }

  pub fn reset_operator(&mut self) {
    self.operator = None;
  }

  pub fn set_time_zone(&mut self, time_zone: String) {
    self.time_zone = Some(time_zone);
  }

  pub fn with_time_zone(mut self, time_zone: String) -> AlertrequestConditions {
    self.time_zone = Some(time_zone);
    self
  }

  pub fn time_zone(&self) -> Option<&String> {
    self.time_zone.as_ref()
  }

  pub fn reset_time_zone(&mut self) {
    self.time_zone = None;
  }

  pub fn set_trigger_method(&mut self, trigger_method: String) {
    self.trigger_method = Some(trigger_method);
  }

  pub fn with_trigger_method(mut self, trigger_method: String) -> AlertrequestConditions {
    self.trigger_method = Some(trigger_method);
    self
  }

  pub fn trigger_method(&self) -> Option<&String> {
    self.trigger_method.as_ref()
  }

  pub fn reset_trigger_method(&mut self) {
    self.trigger_method = None;
  }

  pub fn set__type(&mut self, _type: i32) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: i32) -> AlertrequestConditions {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&i32> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_value(&mut self, value: String) {
    self.value = Some(value);
  }

  pub fn with_value(mut self, value: String) -> AlertrequestConditions {
    self.value = Some(value);
    self
  }

  pub fn value(&self) -> Option<&String> {
    self.value.as_ref()
  }

  pub fn reset_value(&mut self) {
    self.value = None;
  }

}




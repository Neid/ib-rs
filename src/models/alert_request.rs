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
pub struct AlertRequest {
  /// The message you want to receive via email or text message
  #[serde(rename = "alertMessage")]
  alert_message: Option<String>,
  /// name of alert
  #[serde(rename = "alertName")]
  alert_name: Option<String>,
  /// whether alert is repeatable or not, so value can only be 0 or 1, this has to be 1 for MTA alert
  #[serde(rename = "alertRepeatable")]
  alert_repeatable: Option<i32>,
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::AlertrequestConditions>>,
  /// email address to receive alert
  #[serde(rename = "email")]
  email: Option<String>,
  /// format, YYYYMMDD-HH:mm:ss, please NOTE this will only work when tif is GTD 
  #[serde(rename = "expireTime")]
  expire_time: Option<String>,
  /// value can only be 0 or 1, set to 1 to enable the alert only in IBKR mobile 
  #[serde(rename = "iTWSOrdersOnly")]
  i_tws_orders_only: Option<i32>,
  /// orderId is required when modifying alert. You can get it from /iserver/account/:accountId/alerts 
  #[serde(rename = "orderId")]
  order_id: Option<i32>,
  /// value can only be 0 or 1, set to 1 if the alert can be triggered outside regular trading hours. 
  #[serde(rename = "outsideRth")]
  outside_rth: Option<i32>,
  /// audio message to play when alert is triggered
  #[serde(rename = "playAudio")]
  play_audio: Option<String>,
  /// whether allowing to send email or not, so value can only be 0 or 1, 
  #[serde(rename = "sendMessage")]
  send_message: Option<i32>,
  /// value can only be 0 or 1, set to 1 to allow to show alert in pop-ups
  #[serde(rename = "showPopup")]
  show_popup: Option<i32>,
  /// time in force, can only be GTC or GTD
  #[serde(rename = "tif")]
  tif: Option<String>,
  /// for MTA alert only, each user has a unique toolId and it will stay the same, do not send for normal alert 
  #[serde(rename = "toolId")]
  tool_id: Option<i32>
}

impl AlertRequest {
  pub fn new() -> AlertRequest {
    AlertRequest {
      alert_message: None,
      alert_name: None,
      alert_repeatable: None,
      conditions: None,
      email: None,
      expire_time: None,
      i_tws_orders_only: None,
      order_id: None,
      outside_rth: None,
      play_audio: None,
      send_message: None,
      show_popup: None,
      tif: None,
      tool_id: None
    }
  }

  pub fn set_alert_message(&mut self, alert_message: String) {
    self.alert_message = Some(alert_message);
  }

  pub fn with_alert_message(mut self, alert_message: String) -> AlertRequest {
    self.alert_message = Some(alert_message);
    self
  }

  pub fn alert_message(&self) -> Option<&String> {
    self.alert_message.as_ref()
  }

  pub fn reset_alert_message(&mut self) {
    self.alert_message = None;
  }

  pub fn set_alert_name(&mut self, alert_name: String) {
    self.alert_name = Some(alert_name);
  }

  pub fn with_alert_name(mut self, alert_name: String) -> AlertRequest {
    self.alert_name = Some(alert_name);
    self
  }

  pub fn alert_name(&self) -> Option<&String> {
    self.alert_name.as_ref()
  }

  pub fn reset_alert_name(&mut self) {
    self.alert_name = None;
  }

  pub fn set_alert_repeatable(&mut self, alert_repeatable: i32) {
    self.alert_repeatable = Some(alert_repeatable);
  }

  pub fn with_alert_repeatable(mut self, alert_repeatable: i32) -> AlertRequest {
    self.alert_repeatable = Some(alert_repeatable);
    self
  }

  pub fn alert_repeatable(&self) -> Option<&i32> {
    self.alert_repeatable.as_ref()
  }

  pub fn reset_alert_repeatable(&mut self) {
    self.alert_repeatable = None;
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::AlertrequestConditions>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::AlertrequestConditions>) -> AlertRequest {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::AlertrequestConditions>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> AlertRequest {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_expire_time(&mut self, expire_time: String) {
    self.expire_time = Some(expire_time);
  }

  pub fn with_expire_time(mut self, expire_time: String) -> AlertRequest {
    self.expire_time = Some(expire_time);
    self
  }

  pub fn expire_time(&self) -> Option<&String> {
    self.expire_time.as_ref()
  }

  pub fn reset_expire_time(&mut self) {
    self.expire_time = None;
  }

  pub fn set_i_tws_orders_only(&mut self, i_tws_orders_only: i32) {
    self.i_tws_orders_only = Some(i_tws_orders_only);
  }

  pub fn with_i_tws_orders_only(mut self, i_tws_orders_only: i32) -> AlertRequest {
    self.i_tws_orders_only = Some(i_tws_orders_only);
    self
  }

  pub fn i_tws_orders_only(&self) -> Option<&i32> {
    self.i_tws_orders_only.as_ref()
  }

  pub fn reset_i_tws_orders_only(&mut self) {
    self.i_tws_orders_only = None;
  }

  pub fn set_order_id(&mut self, order_id: i32) {
    self.order_id = Some(order_id);
  }

  pub fn with_order_id(mut self, order_id: i32) -> AlertRequest {
    self.order_id = Some(order_id);
    self
  }

  pub fn order_id(&self) -> Option<&i32> {
    self.order_id.as_ref()
  }

  pub fn reset_order_id(&mut self) {
    self.order_id = None;
  }

  pub fn set_outside_rth(&mut self, outside_rth: i32) {
    self.outside_rth = Some(outside_rth);
  }

  pub fn with_outside_rth(mut self, outside_rth: i32) -> AlertRequest {
    self.outside_rth = Some(outside_rth);
    self
  }

  pub fn outside_rth(&self) -> Option<&i32> {
    self.outside_rth.as_ref()
  }

  pub fn reset_outside_rth(&mut self) {
    self.outside_rth = None;
  }

  pub fn set_play_audio(&mut self, play_audio: String) {
    self.play_audio = Some(play_audio);
  }

  pub fn with_play_audio(mut self, play_audio: String) -> AlertRequest {
    self.play_audio = Some(play_audio);
    self
  }

  pub fn play_audio(&self) -> Option<&String> {
    self.play_audio.as_ref()
  }

  pub fn reset_play_audio(&mut self) {
    self.play_audio = None;
  }

  pub fn set_send_message(&mut self, send_message: i32) {
    self.send_message = Some(send_message);
  }

  pub fn with_send_message(mut self, send_message: i32) -> AlertRequest {
    self.send_message = Some(send_message);
    self
  }

  pub fn send_message(&self) -> Option<&i32> {
    self.send_message.as_ref()
  }

  pub fn reset_send_message(&mut self) {
    self.send_message = None;
  }

  pub fn set_show_popup(&mut self, show_popup: i32) {
    self.show_popup = Some(show_popup);
  }

  pub fn with_show_popup(mut self, show_popup: i32) -> AlertRequest {
    self.show_popup = Some(show_popup);
    self
  }

  pub fn show_popup(&self) -> Option<&i32> {
    self.show_popup.as_ref()
  }

  pub fn reset_show_popup(&mut self) {
    self.show_popup = None;
  }

  pub fn set_tif(&mut self, tif: String) {
    self.tif = Some(tif);
  }

  pub fn with_tif(mut self, tif: String) -> AlertRequest {
    self.tif = Some(tif);
    self
  }

  pub fn tif(&self) -> Option<&String> {
    self.tif.as_ref()
  }

  pub fn reset_tif(&mut self) {
    self.tif = None;
  }

  pub fn set_tool_id(&mut self, tool_id: i32) {
    self.tool_id = Some(tool_id);
  }

  pub fn with_tool_id(mut self, tool_id: i32) -> AlertRequest {
    self.tool_id = Some(tool_id);
    self
  }

  pub fn tool_id(&self) -> Option<&i32> {
    self.tool_id.as_ref()
  }

  pub fn reset_tool_id(&mut self) {
    self.tool_id = None;
  }

}




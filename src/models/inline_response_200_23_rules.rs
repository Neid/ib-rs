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
pub struct InlineResponse20023Rules {
  #[serde(rename = "algoEligible")]
  algo_eligible: Option<bool>,
  /// Cash currency for the contract
  #[serde(rename = "cashCcy")]
  cash_ccy: Option<String>,
  /// Increment value for cash quantity
  #[serde(rename = "cashQtyIncr")]
  cash_qty_incr: Option<f32>,
  /// cash value
  #[serde(rename = "cashSize")]
  cash_size: Option<i32>,
  #[serde(rename = "cqtTypes")]
  cqt_types: Option<Vec<::models::InlineResponse20023CqtTypes>>,
  /// Default quantity
  #[serde(rename = "defaultSize")]
  default_size: Option<i32>,
  /// Default time in force value
  #[serde(rename = "defaultTIF")]
  default_tif: Option<String>,
  #[serde(rename = "displaySize")]
  display_size: Option<f32>,
  #[serde(rename = "error")]
  error: Option<String>,
  /// decimal places for fractional order size
  #[serde(rename = "fraqInt")]
  fraq_int: Option<f32>,
  #[serde(rename = "fraqTypes")]
  fraq_types: Option<Vec<::models::InlineResponse20023FraqTypes>>,
  #[serde(rename = "ibalgoTypes")]
  ibalgo_types: Option<Vec<::models::InlineResponse20023IbalgoTypes>>,
  /// Price increment value
  #[serde(rename = "increment")]
  increment: Option<f32>,
  /// Number of digits for price increment
  #[serde(rename = "incrementDigits")]
  increment_digits: Option<i32>,
  /// Limit price
  #[serde(rename = "limitPrice")]
  limit_price: Option<f32>,
  /// trading negative price support
  #[serde(rename = "negativeCapable")]
  negative_capable: Option<bool>,
  /// If object returned will provide the defaults based on user settings
  #[serde(rename = "orderDefaults")]
  order_defaults: Option<Vec<::models::InlineResponse20023OrderDefaults>>,
  /// Order origin designation for US securities options and Options Clearing Corporation
  #[serde(rename = "orderOrigination")]
  order_origination: Option<f32>,
  #[serde(rename = "orderTypes")]
  order_types: Option<Vec<::models::InlineResponse20023OrderTypes>>,
  #[serde(rename = "orderTypesOutside")]
  order_types_outside: Option<Vec<::models::InlineResponse20023OrderTypesOutside>>,
  /// order preview required
  #[serde(rename = "preview")]
  preview: Option<bool>,
  /// Price Magnifier
  #[serde(rename = "priceMagnifier")]
  price_magnifier: Option<f32>,
  /// increment quantity value
  #[serde(rename = "sizeIncrement")]
  size_increment: Option<i32>,
  /// Stop price
  #[serde(rename = "stopprice")]
  stopprice: Option<f32>,
  #[serde(rename = "tifTypes")]
  tif_types: Option<Vec<::models::InlineResponse20023TifTypes>>
}

impl InlineResponse20023Rules {
  pub fn new() -> InlineResponse20023Rules {
    InlineResponse20023Rules {
      algo_eligible: None,
      cash_ccy: None,
      cash_qty_incr: None,
      cash_size: None,
      cqt_types: None,
      default_size: None,
      default_tif: None,
      display_size: None,
      error: None,
      fraq_int: None,
      fraq_types: None,
      ibalgo_types: None,
      increment: None,
      increment_digits: None,
      limit_price: None,
      negative_capable: None,
      order_defaults: None,
      order_origination: None,
      order_types: None,
      order_types_outside: None,
      preview: None,
      price_magnifier: None,
      size_increment: None,
      stopprice: None,
      tif_types: None
    }
  }

  pub fn set_algo_eligible(&mut self, algo_eligible: bool) {
    self.algo_eligible = Some(algo_eligible);
  }

  pub fn with_algo_eligible(mut self, algo_eligible: bool) -> InlineResponse20023Rules {
    self.algo_eligible = Some(algo_eligible);
    self
  }

  pub fn algo_eligible(&self) -> Option<&bool> {
    self.algo_eligible.as_ref()
  }

  pub fn reset_algo_eligible(&mut self) {
    self.algo_eligible = None;
  }

  pub fn set_cash_ccy(&mut self, cash_ccy: String) {
    self.cash_ccy = Some(cash_ccy);
  }

  pub fn with_cash_ccy(mut self, cash_ccy: String) -> InlineResponse20023Rules {
    self.cash_ccy = Some(cash_ccy);
    self
  }

  pub fn cash_ccy(&self) -> Option<&String> {
    self.cash_ccy.as_ref()
  }

  pub fn reset_cash_ccy(&mut self) {
    self.cash_ccy = None;
  }

  pub fn set_cash_qty_incr(&mut self, cash_qty_incr: f32) {
    self.cash_qty_incr = Some(cash_qty_incr);
  }

  pub fn with_cash_qty_incr(mut self, cash_qty_incr: f32) -> InlineResponse20023Rules {
    self.cash_qty_incr = Some(cash_qty_incr);
    self
  }

  pub fn cash_qty_incr(&self) -> Option<&f32> {
    self.cash_qty_incr.as_ref()
  }

  pub fn reset_cash_qty_incr(&mut self) {
    self.cash_qty_incr = None;
  }

  pub fn set_cash_size(&mut self, cash_size: i32) {
    self.cash_size = Some(cash_size);
  }

  pub fn with_cash_size(mut self, cash_size: i32) -> InlineResponse20023Rules {
    self.cash_size = Some(cash_size);
    self
  }

  pub fn cash_size(&self) -> Option<&i32> {
    self.cash_size.as_ref()
  }

  pub fn reset_cash_size(&mut self) {
    self.cash_size = None;
  }

  pub fn set_cqt_types(&mut self, cqt_types: Vec<::models::InlineResponse20023CqtTypes>) {
    self.cqt_types = Some(cqt_types);
  }

  pub fn with_cqt_types(mut self, cqt_types: Vec<::models::InlineResponse20023CqtTypes>) -> InlineResponse20023Rules {
    self.cqt_types = Some(cqt_types);
    self
  }

  pub fn cqt_types(&self) -> Option<&Vec<::models::InlineResponse20023CqtTypes>> {
    self.cqt_types.as_ref()
  }

  pub fn reset_cqt_types(&mut self) {
    self.cqt_types = None;
  }

  pub fn set_default_size(&mut self, default_size: i32) {
    self.default_size = Some(default_size);
  }

  pub fn with_default_size(mut self, default_size: i32) -> InlineResponse20023Rules {
    self.default_size = Some(default_size);
    self
  }

  pub fn default_size(&self) -> Option<&i32> {
    self.default_size.as_ref()
  }

  pub fn reset_default_size(&mut self) {
    self.default_size = None;
  }

  pub fn set_default_tif(&mut self, default_tif: String) {
    self.default_tif = Some(default_tif);
  }

  pub fn with_default_tif(mut self, default_tif: String) -> InlineResponse20023Rules {
    self.default_tif = Some(default_tif);
    self
  }

  pub fn default_tif(&self) -> Option<&String> {
    self.default_tif.as_ref()
  }

  pub fn reset_default_tif(&mut self) {
    self.default_tif = None;
  }

  pub fn set_display_size(&mut self, display_size: f32) {
    self.display_size = Some(display_size);
  }

  pub fn with_display_size(mut self, display_size: f32) -> InlineResponse20023Rules {
    self.display_size = Some(display_size);
    self
  }

  pub fn display_size(&self) -> Option<&f32> {
    self.display_size.as_ref()
  }

  pub fn reset_display_size(&mut self) {
    self.display_size = None;
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> InlineResponse20023Rules {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

  pub fn set_fraq_int(&mut self, fraq_int: f32) {
    self.fraq_int = Some(fraq_int);
  }

  pub fn with_fraq_int(mut self, fraq_int: f32) -> InlineResponse20023Rules {
    self.fraq_int = Some(fraq_int);
    self
  }

  pub fn fraq_int(&self) -> Option<&f32> {
    self.fraq_int.as_ref()
  }

  pub fn reset_fraq_int(&mut self) {
    self.fraq_int = None;
  }

  pub fn set_fraq_types(&mut self, fraq_types: Vec<::models::InlineResponse20023FraqTypes>) {
    self.fraq_types = Some(fraq_types);
  }

  pub fn with_fraq_types(mut self, fraq_types: Vec<::models::InlineResponse20023FraqTypes>) -> InlineResponse20023Rules {
    self.fraq_types = Some(fraq_types);
    self
  }

  pub fn fraq_types(&self) -> Option<&Vec<::models::InlineResponse20023FraqTypes>> {
    self.fraq_types.as_ref()
  }

  pub fn reset_fraq_types(&mut self) {
    self.fraq_types = None;
  }

  pub fn set_ibalgo_types(&mut self, ibalgo_types: Vec<::models::InlineResponse20023IbalgoTypes>) {
    self.ibalgo_types = Some(ibalgo_types);
  }

  pub fn with_ibalgo_types(mut self, ibalgo_types: Vec<::models::InlineResponse20023IbalgoTypes>) -> InlineResponse20023Rules {
    self.ibalgo_types = Some(ibalgo_types);
    self
  }

  pub fn ibalgo_types(&self) -> Option<&Vec<::models::InlineResponse20023IbalgoTypes>> {
    self.ibalgo_types.as_ref()
  }

  pub fn reset_ibalgo_types(&mut self) {
    self.ibalgo_types = None;
  }

  pub fn set_increment(&mut self, increment: f32) {
    self.increment = Some(increment);
  }

  pub fn with_increment(mut self, increment: f32) -> InlineResponse20023Rules {
    self.increment = Some(increment);
    self
  }

  pub fn increment(&self) -> Option<&f32> {
    self.increment.as_ref()
  }

  pub fn reset_increment(&mut self) {
    self.increment = None;
  }

  pub fn set_increment_digits(&mut self, increment_digits: i32) {
    self.increment_digits = Some(increment_digits);
  }

  pub fn with_increment_digits(mut self, increment_digits: i32) -> InlineResponse20023Rules {
    self.increment_digits = Some(increment_digits);
    self
  }

  pub fn increment_digits(&self) -> Option<&i32> {
    self.increment_digits.as_ref()
  }

  pub fn reset_increment_digits(&mut self) {
    self.increment_digits = None;
  }

  pub fn set_limit_price(&mut self, limit_price: f32) {
    self.limit_price = Some(limit_price);
  }

  pub fn with_limit_price(mut self, limit_price: f32) -> InlineResponse20023Rules {
    self.limit_price = Some(limit_price);
    self
  }

  pub fn limit_price(&self) -> Option<&f32> {
    self.limit_price.as_ref()
  }

  pub fn reset_limit_price(&mut self) {
    self.limit_price = None;
  }

  pub fn set_negative_capable(&mut self, negative_capable: bool) {
    self.negative_capable = Some(negative_capable);
  }

  pub fn with_negative_capable(mut self, negative_capable: bool) -> InlineResponse20023Rules {
    self.negative_capable = Some(negative_capable);
    self
  }

  pub fn negative_capable(&self) -> Option<&bool> {
    self.negative_capable.as_ref()
  }

  pub fn reset_negative_capable(&mut self) {
    self.negative_capable = None;
  }

  pub fn set_order_defaults(&mut self, order_defaults: Vec<::models::InlineResponse20023OrderDefaults>) {
    self.order_defaults = Some(order_defaults);
  }

  pub fn with_order_defaults(mut self, order_defaults: Vec<::models::InlineResponse20023OrderDefaults>) -> InlineResponse20023Rules {
    self.order_defaults = Some(order_defaults);
    self
  }

  pub fn order_defaults(&self) -> Option<&Vec<::models::InlineResponse20023OrderDefaults>> {
    self.order_defaults.as_ref()
  }

  pub fn reset_order_defaults(&mut self) {
    self.order_defaults = None;
  }

  pub fn set_order_origination(&mut self, order_origination: f32) {
    self.order_origination = Some(order_origination);
  }

  pub fn with_order_origination(mut self, order_origination: f32) -> InlineResponse20023Rules {
    self.order_origination = Some(order_origination);
    self
  }

  pub fn order_origination(&self) -> Option<&f32> {
    self.order_origination.as_ref()
  }

  pub fn reset_order_origination(&mut self) {
    self.order_origination = None;
  }

  pub fn set_order_types(&mut self, order_types: Vec<::models::InlineResponse20023OrderTypes>) {
    self.order_types = Some(order_types);
  }

  pub fn with_order_types(mut self, order_types: Vec<::models::InlineResponse20023OrderTypes>) -> InlineResponse20023Rules {
    self.order_types = Some(order_types);
    self
  }

  pub fn order_types(&self) -> Option<&Vec<::models::InlineResponse20023OrderTypes>> {
    self.order_types.as_ref()
  }

  pub fn reset_order_types(&mut self) {
    self.order_types = None;
  }

  pub fn set_order_types_outside(&mut self, order_types_outside: Vec<::models::InlineResponse20023OrderTypesOutside>) {
    self.order_types_outside = Some(order_types_outside);
  }

  pub fn with_order_types_outside(mut self, order_types_outside: Vec<::models::InlineResponse20023OrderTypesOutside>) -> InlineResponse20023Rules {
    self.order_types_outside = Some(order_types_outside);
    self
  }

  pub fn order_types_outside(&self) -> Option<&Vec<::models::InlineResponse20023OrderTypesOutside>> {
    self.order_types_outside.as_ref()
  }

  pub fn reset_order_types_outside(&mut self) {
    self.order_types_outside = None;
  }

  pub fn set_preview(&mut self, preview: bool) {
    self.preview = Some(preview);
  }

  pub fn with_preview(mut self, preview: bool) -> InlineResponse20023Rules {
    self.preview = Some(preview);
    self
  }

  pub fn preview(&self) -> Option<&bool> {
    self.preview.as_ref()
  }

  pub fn reset_preview(&mut self) {
    self.preview = None;
  }

  pub fn set_price_magnifier(&mut self, price_magnifier: f32) {
    self.price_magnifier = Some(price_magnifier);
  }

  pub fn with_price_magnifier(mut self, price_magnifier: f32) -> InlineResponse20023Rules {
    self.price_magnifier = Some(price_magnifier);
    self
  }

  pub fn price_magnifier(&self) -> Option<&f32> {
    self.price_magnifier.as_ref()
  }

  pub fn reset_price_magnifier(&mut self) {
    self.price_magnifier = None;
  }

  pub fn set_size_increment(&mut self, size_increment: i32) {
    self.size_increment = Some(size_increment);
  }

  pub fn with_size_increment(mut self, size_increment: i32) -> InlineResponse20023Rules {
    self.size_increment = Some(size_increment);
    self
  }

  pub fn size_increment(&self) -> Option<&i32> {
    self.size_increment.as_ref()
  }

  pub fn reset_size_increment(&mut self) {
    self.size_increment = None;
  }

  pub fn set_stopprice(&mut self, stopprice: f32) {
    self.stopprice = Some(stopprice);
  }

  pub fn with_stopprice(mut self, stopprice: f32) -> InlineResponse20023Rules {
    self.stopprice = Some(stopprice);
    self
  }

  pub fn stopprice(&self) -> Option<&f32> {
    self.stopprice.as_ref()
  }

  pub fn reset_stopprice(&mut self) {
    self.stopprice = None;
  }

  pub fn set_tif_types(&mut self, tif_types: Vec<::models::InlineResponse20023TifTypes>) {
    self.tif_types = Some(tif_types);
  }

  pub fn with_tif_types(mut self, tif_types: Vec<::models::InlineResponse20023TifTypes>) -> InlineResponse20023Rules {
    self.tif_types = Some(tif_types);
    self
  }

  pub fn tif_types(&self) -> Option<&Vec<::models::InlineResponse20023TifTypes>> {
    self.tif_types.as_ref()
  }

  pub fn reset_tif_types(&mut self) {
    self.tif_types = None;
  }

}




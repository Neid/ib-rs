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
pub struct InlineResponse20014Orders {
  /// Account number
  #[serde(rename = "acct")]
  acct: Option<String>,
  /// Background color in hex format
  #[serde(rename = "bgColor")]
  bg_color: Option<String>,
  /// Cash currency
  #[serde(rename = "cashCcy")]
  cash_ccy: Option<String>,
  /// Company Name
  #[serde(rename = "companyName")]
  company_name: Option<String>,
  /// Contract identifier
  #[serde(rename = "conid")]
  conid: Option<f32>,
  /// conid and exchange. Format supports conid or conid@exchange
  #[serde(rename = "conidex")]
  conidex: Option<String>,
  /// Formatted ticker description
  #[serde(rename = "description1")]
  description1: Option<String>,
  /// Foreground color in hex format
  #[serde(rename = "fgColor")]
  fg_color: Option<String>,
  /// Quantity filled
  #[serde(rename = "filledQuantity")]
  filled_quantity: Option<f32>,
  /// Last status update in format YYMMDDhhmms based in GMT
  #[serde(rename = "lastExecutionTime")]
  last_execution_time: Option<f32>,
  /// Last status update unix time in ms
  #[serde(rename = "lastExecutionTime_r")]
  last_execution_time_r: Option<f32>,
  /// Listing Exchange
  #[serde(rename = "listingExchange")]
  listing_exchange: Option<String>,
  /// Order description
  #[serde(rename = "orderDesc")]
  order_desc: Option<String>,
  /// Order identifier
  #[serde(rename = "orderId")]
  order_id: Option<String>,
  /// Order type
  #[serde(rename = "orderType")]
  order_type: Option<String>,
  /// Order reference
  #[serde(rename = "order_ref")]
  order_ref: Option<String>,
  /// Original order type
  #[serde(rename = "origOrderType")]
  orig_order_type: Option<String>,
  /// Price of order
  #[serde(rename = "price")]
  price: Option<f32>,
  /// Quantity remaining
  #[serde(rename = "remainingQuantity")]
  remaining_quantity: Option<f32>,
  /// Asset class
  #[serde(rename = "secType")]
  sec_type: Option<String>,
  /// The side of the market of the order.  * BUY: Buy contract near posted ask price  * SELL: Sell contract near posted bid price  * ASSN: Option Assignment, if BUYSELL=BUY and OptionType=PUT or BUYSELL=SELL and OptionType=CALL  * EXER: Option Exercise, if BUYSELL=SELL and OptionType=PUT or BUYSELL=BUY and OptionType=CALL 
  #[serde(rename = "side")]
  side: Option<String>,
  /// Quantity outstanding and total quantity concatenated with forward slash separator
  #[serde(rename = "sizeAndFills")]
  size_and_fills: Option<f32>,
  /// Status of the order
  #[serde(rename = "status")]
  status: Option<String>,
  /// Supports Tax Optimization with 0 for no and 1 for yes
  #[serde(rename = "supportsTaxOpt")]
  supports_tax_opt: Option<f32>,
  /// Underlying symbol
  #[serde(rename = "ticker")]
  ticker: Option<String>,
  /// Time in force
  #[serde(rename = "timeInForce")]
  time_in_force: Option<String>
}

impl InlineResponse20014Orders {
  pub fn new() -> InlineResponse20014Orders {
    InlineResponse20014Orders {
      acct: None,
      bg_color: None,
      cash_ccy: None,
      company_name: None,
      conid: None,
      conidex: None,
      description1: None,
      fg_color: None,
      filled_quantity: None,
      last_execution_time: None,
      last_execution_time_r: None,
      listing_exchange: None,
      order_desc: None,
      order_id: None,
      order_type: None,
      order_ref: None,
      orig_order_type: None,
      price: None,
      remaining_quantity: None,
      sec_type: None,
      side: None,
      size_and_fills: None,
      status: None,
      supports_tax_opt: None,
      ticker: None,
      time_in_force: None
    }
  }

  pub fn set_acct(&mut self, acct: String) {
    self.acct = Some(acct);
  }

  pub fn with_acct(mut self, acct: String) -> InlineResponse20014Orders {
    self.acct = Some(acct);
    self
  }

  pub fn acct(&self) -> Option<&String> {
    self.acct.as_ref()
  }

  pub fn reset_acct(&mut self) {
    self.acct = None;
  }

  pub fn set_bg_color(&mut self, bg_color: String) {
    self.bg_color = Some(bg_color);
  }

  pub fn with_bg_color(mut self, bg_color: String) -> InlineResponse20014Orders {
    self.bg_color = Some(bg_color);
    self
  }

  pub fn bg_color(&self) -> Option<&String> {
    self.bg_color.as_ref()
  }

  pub fn reset_bg_color(&mut self) {
    self.bg_color = None;
  }

  pub fn set_cash_ccy(&mut self, cash_ccy: String) {
    self.cash_ccy = Some(cash_ccy);
  }

  pub fn with_cash_ccy(mut self, cash_ccy: String) -> InlineResponse20014Orders {
    self.cash_ccy = Some(cash_ccy);
    self
  }

  pub fn cash_ccy(&self) -> Option<&String> {
    self.cash_ccy.as_ref()
  }

  pub fn reset_cash_ccy(&mut self) {
    self.cash_ccy = None;
  }

  pub fn set_company_name(&mut self, company_name: String) {
    self.company_name = Some(company_name);
  }

  pub fn with_company_name(mut self, company_name: String) -> InlineResponse20014Orders {
    self.company_name = Some(company_name);
    self
  }

  pub fn company_name(&self) -> Option<&String> {
    self.company_name.as_ref()
  }

  pub fn reset_company_name(&mut self) {
    self.company_name = None;
  }

  pub fn set_conid(&mut self, conid: f32) {
    self.conid = Some(conid);
  }

  pub fn with_conid(mut self, conid: f32) -> InlineResponse20014Orders {
    self.conid = Some(conid);
    self
  }

  pub fn conid(&self) -> Option<&f32> {
    self.conid.as_ref()
  }

  pub fn reset_conid(&mut self) {
    self.conid = None;
  }

  pub fn set_conidex(&mut self, conidex: String) {
    self.conidex = Some(conidex);
  }

  pub fn with_conidex(mut self, conidex: String) -> InlineResponse20014Orders {
    self.conidex = Some(conidex);
    self
  }

  pub fn conidex(&self) -> Option<&String> {
    self.conidex.as_ref()
  }

  pub fn reset_conidex(&mut self) {
    self.conidex = None;
  }

  pub fn set_description1(&mut self, description1: String) {
    self.description1 = Some(description1);
  }

  pub fn with_description1(mut self, description1: String) -> InlineResponse20014Orders {
    self.description1 = Some(description1);
    self
  }

  pub fn description1(&self) -> Option<&String> {
    self.description1.as_ref()
  }

  pub fn reset_description1(&mut self) {
    self.description1 = None;
  }

  pub fn set_fg_color(&mut self, fg_color: String) {
    self.fg_color = Some(fg_color);
  }

  pub fn with_fg_color(mut self, fg_color: String) -> InlineResponse20014Orders {
    self.fg_color = Some(fg_color);
    self
  }

  pub fn fg_color(&self) -> Option<&String> {
    self.fg_color.as_ref()
  }

  pub fn reset_fg_color(&mut self) {
    self.fg_color = None;
  }

  pub fn set_filled_quantity(&mut self, filled_quantity: f32) {
    self.filled_quantity = Some(filled_quantity);
  }

  pub fn with_filled_quantity(mut self, filled_quantity: f32) -> InlineResponse20014Orders {
    self.filled_quantity = Some(filled_quantity);
    self
  }

  pub fn filled_quantity(&self) -> Option<&f32> {
    self.filled_quantity.as_ref()
  }

  pub fn reset_filled_quantity(&mut self) {
    self.filled_quantity = None;
  }

  pub fn set_last_execution_time(&mut self, last_execution_time: f32) {
    self.last_execution_time = Some(last_execution_time);
  }

  pub fn with_last_execution_time(mut self, last_execution_time: f32) -> InlineResponse20014Orders {
    self.last_execution_time = Some(last_execution_time);
    self
  }

  pub fn last_execution_time(&self) -> Option<&f32> {
    self.last_execution_time.as_ref()
  }

  pub fn reset_last_execution_time(&mut self) {
    self.last_execution_time = None;
  }

  pub fn set_last_execution_time_r(&mut self, last_execution_time_r: f32) {
    self.last_execution_time_r = Some(last_execution_time_r);
  }

  pub fn with_last_execution_time_r(mut self, last_execution_time_r: f32) -> InlineResponse20014Orders {
    self.last_execution_time_r = Some(last_execution_time_r);
    self
  }

  pub fn last_execution_time_r(&self) -> Option<&f32> {
    self.last_execution_time_r.as_ref()
  }

  pub fn reset_last_execution_time_r(&mut self) {
    self.last_execution_time_r = None;
  }

  pub fn set_listing_exchange(&mut self, listing_exchange: String) {
    self.listing_exchange = Some(listing_exchange);
  }

  pub fn with_listing_exchange(mut self, listing_exchange: String) -> InlineResponse20014Orders {
    self.listing_exchange = Some(listing_exchange);
    self
  }

  pub fn listing_exchange(&self) -> Option<&String> {
    self.listing_exchange.as_ref()
  }

  pub fn reset_listing_exchange(&mut self) {
    self.listing_exchange = None;
  }

  pub fn set_order_desc(&mut self, order_desc: String) {
    self.order_desc = Some(order_desc);
  }

  pub fn with_order_desc(mut self, order_desc: String) -> InlineResponse20014Orders {
    self.order_desc = Some(order_desc);
    self
  }

  pub fn order_desc(&self) -> Option<&String> {
    self.order_desc.as_ref()
  }

  pub fn reset_order_desc(&mut self) {
    self.order_desc = None;
  }

  pub fn set_order_id(&mut self, order_id: String) {
    self.order_id = Some(order_id);
  }

  pub fn with_order_id(mut self, order_id: String) -> InlineResponse20014Orders {
    self.order_id = Some(order_id);
    self
  }

  pub fn order_id(&self) -> Option<&String> {
    self.order_id.as_ref()
  }

  pub fn reset_order_id(&mut self) {
    self.order_id = None;
  }

  pub fn set_order_type(&mut self, order_type: String) {
    self.order_type = Some(order_type);
  }

  pub fn with_order_type(mut self, order_type: String) -> InlineResponse20014Orders {
    self.order_type = Some(order_type);
    self
  }

  pub fn order_type(&self) -> Option<&String> {
    self.order_type.as_ref()
  }

  pub fn reset_order_type(&mut self) {
    self.order_type = None;
  }

  pub fn set_order_ref(&mut self, order_ref: String) {
    self.order_ref = Some(order_ref);
  }

  pub fn with_order_ref(mut self, order_ref: String) -> InlineResponse20014Orders {
    self.order_ref = Some(order_ref);
    self
  }

  pub fn order_ref(&self) -> Option<&String> {
    self.order_ref.as_ref()
  }

  pub fn reset_order_ref(&mut self) {
    self.order_ref = None;
  }

  pub fn set_orig_order_type(&mut self, orig_order_type: String) {
    self.orig_order_type = Some(orig_order_type);
  }

  pub fn with_orig_order_type(mut self, orig_order_type: String) -> InlineResponse20014Orders {
    self.orig_order_type = Some(orig_order_type);
    self
  }

  pub fn orig_order_type(&self) -> Option<&String> {
    self.orig_order_type.as_ref()
  }

  pub fn reset_orig_order_type(&mut self) {
    self.orig_order_type = None;
  }

  pub fn set_price(&mut self, price: f32) {
    self.price = Some(price);
  }

  pub fn with_price(mut self, price: f32) -> InlineResponse20014Orders {
    self.price = Some(price);
    self
  }

  pub fn price(&self) -> Option<&f32> {
    self.price.as_ref()
  }

  pub fn reset_price(&mut self) {
    self.price = None;
  }

  pub fn set_remaining_quantity(&mut self, remaining_quantity: f32) {
    self.remaining_quantity = Some(remaining_quantity);
  }

  pub fn with_remaining_quantity(mut self, remaining_quantity: f32) -> InlineResponse20014Orders {
    self.remaining_quantity = Some(remaining_quantity);
    self
  }

  pub fn remaining_quantity(&self) -> Option<&f32> {
    self.remaining_quantity.as_ref()
  }

  pub fn reset_remaining_quantity(&mut self) {
    self.remaining_quantity = None;
  }

  pub fn set_sec_type(&mut self, sec_type: String) {
    self.sec_type = Some(sec_type);
  }

  pub fn with_sec_type(mut self, sec_type: String) -> InlineResponse20014Orders {
    self.sec_type = Some(sec_type);
    self
  }

  pub fn sec_type(&self) -> Option<&String> {
    self.sec_type.as_ref()
  }

  pub fn reset_sec_type(&mut self) {
    self.sec_type = None;
  }

  pub fn set_side(&mut self, side: String) {
    self.side = Some(side);
  }

  pub fn with_side(mut self, side: String) -> InlineResponse20014Orders {
    self.side = Some(side);
    self
  }

  pub fn side(&self) -> Option<&String> {
    self.side.as_ref()
  }

  pub fn reset_side(&mut self) {
    self.side = None;
  }

  pub fn set_size_and_fills(&mut self, size_and_fills: f32) {
    self.size_and_fills = Some(size_and_fills);
  }

  pub fn with_size_and_fills(mut self, size_and_fills: f32) -> InlineResponse20014Orders {
    self.size_and_fills = Some(size_and_fills);
    self
  }

  pub fn size_and_fills(&self) -> Option<&f32> {
    self.size_and_fills.as_ref()
  }

  pub fn reset_size_and_fills(&mut self) {
    self.size_and_fills = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> InlineResponse20014Orders {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_supports_tax_opt(&mut self, supports_tax_opt: f32) {
    self.supports_tax_opt = Some(supports_tax_opt);
  }

  pub fn with_supports_tax_opt(mut self, supports_tax_opt: f32) -> InlineResponse20014Orders {
    self.supports_tax_opt = Some(supports_tax_opt);
    self
  }

  pub fn supports_tax_opt(&self) -> Option<&f32> {
    self.supports_tax_opt.as_ref()
  }

  pub fn reset_supports_tax_opt(&mut self) {
    self.supports_tax_opt = None;
  }

  pub fn set_ticker(&mut self, ticker: String) {
    self.ticker = Some(ticker);
  }

  pub fn with_ticker(mut self, ticker: String) -> InlineResponse20014Orders {
    self.ticker = Some(ticker);
    self
  }

  pub fn ticker(&self) -> Option<&String> {
    self.ticker.as_ref()
  }

  pub fn reset_ticker(&mut self) {
    self.ticker = None;
  }

  pub fn set_time_in_force(&mut self, time_in_force: String) {
    self.time_in_force = Some(time_in_force);
  }

  pub fn with_time_in_force(mut self, time_in_force: String) -> InlineResponse20014Orders {
    self.time_in_force = Some(time_in_force);
    self
  }

  pub fn time_in_force(&self) -> Option<&String> {
    self.time_in_force.as_ref()
  }

  pub fn reset_time_in_force(&mut self) {
    self.time_in_force = None;
  }

}




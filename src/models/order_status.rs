/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// OrderStatus : contains all the details of an order

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
  /// account id
  #[serde(rename = "account")]
  account: Option<String>,
  #[serde(rename = "alert_active")]
  alert_active: Option<i32>,
  /// Returns true if contract supports duplicate/opposite side order.
  #[serde(rename = "allowed_duplicate_opposite")]
  allowed_duplicate_opposite: Option<bool>,
  /// List of available chart periods
  #[serde(rename = "available_chart_periods")]
  available_chart_periods: Option<String>,
  /// Background color in hex format
  #[serde(rename = "bg_color")]
  bg_color: Option<String>,
  /// If true not allowed to cancel order
  #[serde(rename = "cannot_cancel_order")]
  cannot_cancel_order: Option<bool>,
  /// type of the child order
  #[serde(rename = "child_order_type")]
  child_order_type: Option<String>,
  /// Contracts company name
  #[serde(rename = "company_name")]
  company_name: Option<String>,
  /// conid and exchange. Format supports conid or conid@exchange
  #[serde(rename = "conidex")]
  conidex: Option<String>,
  /// Format contract name
  #[serde(rename = "contract_description_1")]
  contract_description_1: Option<String>,
  /// Cumulative fill
  #[serde(rename = "cum_fill")]
  cum_fill: Option<String>,
  /// Contract traded currency
  #[serde(rename = "currency")]
  currency: Option<String>,
  /// If true order is de-activated
  #[serde(rename = "deactivate_order")]
  deactivate_order: Option<bool>,
  /// Fields that can be edited in escaped unicode characters
  #[serde(rename = "editable_fields")]
  editable_fields: Option<String>,
  /// Position description to display on chart
  #[serde(rename = "exit_strategy_chart_description")]
  exit_strategy_chart_description: Option<String>,
  /// Position display price
  #[serde(rename = "exit_strategy_display_price")]
  exit_strategy_display_price: Option<String>,
  /// * 1: If your account has position or order for contract * 0: If your account has no position or order for contract 
  #[serde(rename = "exit_strategy_tool_availability")]
  exit_strategy_tool_availability: Option<String>,
  /// Foreground color in hex format
  #[serde(rename = "fg_color")]
  fg_color: Option<String>,
  /// Limit price
  #[serde(rename = "limit_price")]
  limit_price: Option<String>,
  /// Trading Exchange or Venue
  #[serde(rename = "listing_exchange")]
  listing_exchange: Option<String>,
  /// only exists for oca orders, oca orders in same group will have same id
  #[serde(rename = "oca_group_id")]
  oca_group_id: Option<String>,
  #[serde(rename = "option_acct")]
  option_acct: Option<String>,
  /// Format description of order
  #[serde(rename = "order_description")]
  order_description: Option<String>,
  /// order_description with the symbol
  #[serde(rename = "order_description_with_contract")]
  order_description_with_contract: Option<String>,
  /// system generated order id, unique per account
  #[serde(rename = "order_id")]
  order_id: Option<i32>,
  /// If true not allowed to modify order
  #[serde(rename = "order_not_editable")]
  order_not_editable: Option<bool>,
  /// *  PendingSubmit - Indicates the order was sent, but confirmation has not been received that it has been received by the destination.                    Occurs most commonly if an exchange is closed. *  PendingCancel - Indicates that a request has been sent to cancel an order but confirmation has not been received of its cancellation. *  PreSubmitted - Indicates that a simulated order type has been accepted by the IBKR system and that this order has yet to be elected.                   The order is held in the IBKR system until the election criteria are met. At that time the order is transmitted to the order destination as specified. *  Submitted - Indicates that the order has been accepted at the order destination and is working. *  Cancelled - Indicates that the balance of the order has been confirmed cancelled by the IB system.                This could occur unexpectedly when IB or the destination has rejected the order. *  Filled - Indicates that the order has been completely filled. *  Inactive - Indicates the order is not working, for instance if the order was invalid and triggered an error message,               or if the order was to short a security and shares have not yet been located. 
  #[serde(rename = "order_status")]
  order_status: Option<String>,
  /// Description of the order status
  #[serde(rename = "order_status_description")]
  order_status_description: Option<String>,
  /// Time of status update in unix time
  #[serde(rename = "order_time")]
  order_time: Option<String>,
  /// Types of orders
  #[serde(rename = "order_type")]
  order_type: Option<String>,
  /// If true order trades outside regular trading hours
  #[serde(rename = "outside_rth")]
  outside_rth: Option<bool>,
  /// order request id
  #[serde(rename = "request_id")]
  request_id: Option<String>,
  /// Asset class
  #[serde(rename = "sec_type")]
  sec_type: Option<String>,
  /// The side of the market of the order.   * B - Buy contract near posted ask price   * S - Sell contract near posted bid price   * X - Option expired 
  #[serde(rename = "side")]
  side: Option<String>,
  /// Quantity updated
  #[serde(rename = "size")]
  size: Option<String>,
  /// Format fillQuantity\\totalQuantity
  #[serde(rename = "size_and_fills")]
  size_and_fills: Option<String>,
  /// Stop price
  #[serde(rename = "stop_price")]
  stop_price: Option<String>,
  /// order sub-type
  #[serde(rename = "sub_type")]
  sub_type: Option<String>,
  /// Underlying symbol
  #[serde(rename = "symbol")]
  symbol: Option<String>,
  /// Time-in-Force - length of time order will continue working before it is canceled.
  #[serde(rename = "tif")]
  tif: Option<String>,
  /// Total quantity
  #[serde(rename = "total_size")]
  total_size: Option<String>,
  /// If true price management algo is enabled, refer to https://www.interactivebrokers.com/en/index.php?f=43423
  #[serde(rename = "use_price_mgmt_algo")]
  use_price_mgmt_algo: Option<bool>
}

impl OrderStatus {
  /// contains all the details of an order
  pub fn new() -> OrderStatus {
    OrderStatus {
      account: None,
      alert_active: None,
      allowed_duplicate_opposite: None,
      available_chart_periods: None,
      bg_color: None,
      cannot_cancel_order: None,
      child_order_type: None,
      company_name: None,
      conidex: None,
      contract_description_1: None,
      cum_fill: None,
      currency: None,
      deactivate_order: None,
      editable_fields: None,
      exit_strategy_chart_description: None,
      exit_strategy_display_price: None,
      exit_strategy_tool_availability: None,
      fg_color: None,
      limit_price: None,
      listing_exchange: None,
      oca_group_id: None,
      option_acct: None,
      order_description: None,
      order_description_with_contract: None,
      order_id: None,
      order_not_editable: None,
      order_status: None,
      order_status_description: None,
      order_time: None,
      order_type: None,
      outside_rth: None,
      request_id: None,
      sec_type: None,
      side: None,
      size: None,
      size_and_fills: None,
      stop_price: None,
      sub_type: None,
      symbol: None,
      tif: None,
      total_size: None,
      use_price_mgmt_algo: None
    }
  }

  pub fn set_account(&mut self, account: String) {
    self.account = Some(account);
  }

  pub fn with_account(mut self, account: String) -> OrderStatus {
    self.account = Some(account);
    self
  }

  pub fn account(&self) -> Option<&String> {
    self.account.as_ref()
  }

  pub fn reset_account(&mut self) {
    self.account = None;
  }

  pub fn set_alert_active(&mut self, alert_active: i32) {
    self.alert_active = Some(alert_active);
  }

  pub fn with_alert_active(mut self, alert_active: i32) -> OrderStatus {
    self.alert_active = Some(alert_active);
    self
  }

  pub fn alert_active(&self) -> Option<&i32> {
    self.alert_active.as_ref()
  }

  pub fn reset_alert_active(&mut self) {
    self.alert_active = None;
  }

  pub fn set_allowed_duplicate_opposite(&mut self, allowed_duplicate_opposite: bool) {
    self.allowed_duplicate_opposite = Some(allowed_duplicate_opposite);
  }

  pub fn with_allowed_duplicate_opposite(mut self, allowed_duplicate_opposite: bool) -> OrderStatus {
    self.allowed_duplicate_opposite = Some(allowed_duplicate_opposite);
    self
  }

  pub fn allowed_duplicate_opposite(&self) -> Option<&bool> {
    self.allowed_duplicate_opposite.as_ref()
  }

  pub fn reset_allowed_duplicate_opposite(&mut self) {
    self.allowed_duplicate_opposite = None;
  }

  pub fn set_available_chart_periods(&mut self, available_chart_periods: String) {
    self.available_chart_periods = Some(available_chart_periods);
  }

  pub fn with_available_chart_periods(mut self, available_chart_periods: String) -> OrderStatus {
    self.available_chart_periods = Some(available_chart_periods);
    self
  }

  pub fn available_chart_periods(&self) -> Option<&String> {
    self.available_chart_periods.as_ref()
  }

  pub fn reset_available_chart_periods(&mut self) {
    self.available_chart_periods = None;
  }

  pub fn set_bg_color(&mut self, bg_color: String) {
    self.bg_color = Some(bg_color);
  }

  pub fn with_bg_color(mut self, bg_color: String) -> OrderStatus {
    self.bg_color = Some(bg_color);
    self
  }

  pub fn bg_color(&self) -> Option<&String> {
    self.bg_color.as_ref()
  }

  pub fn reset_bg_color(&mut self) {
    self.bg_color = None;
  }

  pub fn set_cannot_cancel_order(&mut self, cannot_cancel_order: bool) {
    self.cannot_cancel_order = Some(cannot_cancel_order);
  }

  pub fn with_cannot_cancel_order(mut self, cannot_cancel_order: bool) -> OrderStatus {
    self.cannot_cancel_order = Some(cannot_cancel_order);
    self
  }

  pub fn cannot_cancel_order(&self) -> Option<&bool> {
    self.cannot_cancel_order.as_ref()
  }

  pub fn reset_cannot_cancel_order(&mut self) {
    self.cannot_cancel_order = None;
  }

  pub fn set_child_order_type(&mut self, child_order_type: String) {
    self.child_order_type = Some(child_order_type);
  }

  pub fn with_child_order_type(mut self, child_order_type: String) -> OrderStatus {
    self.child_order_type = Some(child_order_type);
    self
  }

  pub fn child_order_type(&self) -> Option<&String> {
    self.child_order_type.as_ref()
  }

  pub fn reset_child_order_type(&mut self) {
    self.child_order_type = None;
  }

  pub fn set_company_name(&mut self, company_name: String) {
    self.company_name = Some(company_name);
  }

  pub fn with_company_name(mut self, company_name: String) -> OrderStatus {
    self.company_name = Some(company_name);
    self
  }

  pub fn company_name(&self) -> Option<&String> {
    self.company_name.as_ref()
  }

  pub fn reset_company_name(&mut self) {
    self.company_name = None;
  }

  pub fn set_conidex(&mut self, conidex: String) {
    self.conidex = Some(conidex);
  }

  pub fn with_conidex(mut self, conidex: String) -> OrderStatus {
    self.conidex = Some(conidex);
    self
  }

  pub fn conidex(&self) -> Option<&String> {
    self.conidex.as_ref()
  }

  pub fn reset_conidex(&mut self) {
    self.conidex = None;
  }

  pub fn set_contract_description_1(&mut self, contract_description_1: String) {
    self.contract_description_1 = Some(contract_description_1);
  }

  pub fn with_contract_description_1(mut self, contract_description_1: String) -> OrderStatus {
    self.contract_description_1 = Some(contract_description_1);
    self
  }

  pub fn contract_description_1(&self) -> Option<&String> {
    self.contract_description_1.as_ref()
  }

  pub fn reset_contract_description_1(&mut self) {
    self.contract_description_1 = None;
  }

  pub fn set_cum_fill(&mut self, cum_fill: String) {
    self.cum_fill = Some(cum_fill);
  }

  pub fn with_cum_fill(mut self, cum_fill: String) -> OrderStatus {
    self.cum_fill = Some(cum_fill);
    self
  }

  pub fn cum_fill(&self) -> Option<&String> {
    self.cum_fill.as_ref()
  }

  pub fn reset_cum_fill(&mut self) {
    self.cum_fill = None;
  }

  pub fn set_currency(&mut self, currency: String) {
    self.currency = Some(currency);
  }

  pub fn with_currency(mut self, currency: String) -> OrderStatus {
    self.currency = Some(currency);
    self
  }

  pub fn currency(&self) -> Option<&String> {
    self.currency.as_ref()
  }

  pub fn reset_currency(&mut self) {
    self.currency = None;
  }

  pub fn set_deactivate_order(&mut self, deactivate_order: bool) {
    self.deactivate_order = Some(deactivate_order);
  }

  pub fn with_deactivate_order(mut self, deactivate_order: bool) -> OrderStatus {
    self.deactivate_order = Some(deactivate_order);
    self
  }

  pub fn deactivate_order(&self) -> Option<&bool> {
    self.deactivate_order.as_ref()
  }

  pub fn reset_deactivate_order(&mut self) {
    self.deactivate_order = None;
  }

  pub fn set_editable_fields(&mut self, editable_fields: String) {
    self.editable_fields = Some(editable_fields);
  }

  pub fn with_editable_fields(mut self, editable_fields: String) -> OrderStatus {
    self.editable_fields = Some(editable_fields);
    self
  }

  pub fn editable_fields(&self) -> Option<&String> {
    self.editable_fields.as_ref()
  }

  pub fn reset_editable_fields(&mut self) {
    self.editable_fields = None;
  }

  pub fn set_exit_strategy_chart_description(&mut self, exit_strategy_chart_description: String) {
    self.exit_strategy_chart_description = Some(exit_strategy_chart_description);
  }

  pub fn with_exit_strategy_chart_description(mut self, exit_strategy_chart_description: String) -> OrderStatus {
    self.exit_strategy_chart_description = Some(exit_strategy_chart_description);
    self
  }

  pub fn exit_strategy_chart_description(&self) -> Option<&String> {
    self.exit_strategy_chart_description.as_ref()
  }

  pub fn reset_exit_strategy_chart_description(&mut self) {
    self.exit_strategy_chart_description = None;
  }

  pub fn set_exit_strategy_display_price(&mut self, exit_strategy_display_price: String) {
    self.exit_strategy_display_price = Some(exit_strategy_display_price);
  }

  pub fn with_exit_strategy_display_price(mut self, exit_strategy_display_price: String) -> OrderStatus {
    self.exit_strategy_display_price = Some(exit_strategy_display_price);
    self
  }

  pub fn exit_strategy_display_price(&self) -> Option<&String> {
    self.exit_strategy_display_price.as_ref()
  }

  pub fn reset_exit_strategy_display_price(&mut self) {
    self.exit_strategy_display_price = None;
  }

  pub fn set_exit_strategy_tool_availability(&mut self, exit_strategy_tool_availability: String) {
    self.exit_strategy_tool_availability = Some(exit_strategy_tool_availability);
  }

  pub fn with_exit_strategy_tool_availability(mut self, exit_strategy_tool_availability: String) -> OrderStatus {
    self.exit_strategy_tool_availability = Some(exit_strategy_tool_availability);
    self
  }

  pub fn exit_strategy_tool_availability(&self) -> Option<&String> {
    self.exit_strategy_tool_availability.as_ref()
  }

  pub fn reset_exit_strategy_tool_availability(&mut self) {
    self.exit_strategy_tool_availability = None;
  }

  pub fn set_fg_color(&mut self, fg_color: String) {
    self.fg_color = Some(fg_color);
  }

  pub fn with_fg_color(mut self, fg_color: String) -> OrderStatus {
    self.fg_color = Some(fg_color);
    self
  }

  pub fn fg_color(&self) -> Option<&String> {
    self.fg_color.as_ref()
  }

  pub fn reset_fg_color(&mut self) {
    self.fg_color = None;
  }

  pub fn set_limit_price(&mut self, limit_price: String) {
    self.limit_price = Some(limit_price);
  }

  pub fn with_limit_price(mut self, limit_price: String) -> OrderStatus {
    self.limit_price = Some(limit_price);
    self
  }

  pub fn limit_price(&self) -> Option<&String> {
    self.limit_price.as_ref()
  }

  pub fn reset_limit_price(&mut self) {
    self.limit_price = None;
  }

  pub fn set_listing_exchange(&mut self, listing_exchange: String) {
    self.listing_exchange = Some(listing_exchange);
  }

  pub fn with_listing_exchange(mut self, listing_exchange: String) -> OrderStatus {
    self.listing_exchange = Some(listing_exchange);
    self
  }

  pub fn listing_exchange(&self) -> Option<&String> {
    self.listing_exchange.as_ref()
  }

  pub fn reset_listing_exchange(&mut self) {
    self.listing_exchange = None;
  }

  pub fn set_oca_group_id(&mut self, oca_group_id: String) {
    self.oca_group_id = Some(oca_group_id);
  }

  pub fn with_oca_group_id(mut self, oca_group_id: String) -> OrderStatus {
    self.oca_group_id = Some(oca_group_id);
    self
  }

  pub fn oca_group_id(&self) -> Option<&String> {
    self.oca_group_id.as_ref()
  }

  pub fn reset_oca_group_id(&mut self) {
    self.oca_group_id = None;
  }

  pub fn set_option_acct(&mut self, option_acct: String) {
    self.option_acct = Some(option_acct);
  }

  pub fn with_option_acct(mut self, option_acct: String) -> OrderStatus {
    self.option_acct = Some(option_acct);
    self
  }

  pub fn option_acct(&self) -> Option<&String> {
    self.option_acct.as_ref()
  }

  pub fn reset_option_acct(&mut self) {
    self.option_acct = None;
  }

  pub fn set_order_description(&mut self, order_description: String) {
    self.order_description = Some(order_description);
  }

  pub fn with_order_description(mut self, order_description: String) -> OrderStatus {
    self.order_description = Some(order_description);
    self
  }

  pub fn order_description(&self) -> Option<&String> {
    self.order_description.as_ref()
  }

  pub fn reset_order_description(&mut self) {
    self.order_description = None;
  }

  pub fn set_order_description_with_contract(&mut self, order_description_with_contract: String) {
    self.order_description_with_contract = Some(order_description_with_contract);
  }

  pub fn with_order_description_with_contract(mut self, order_description_with_contract: String) -> OrderStatus {
    self.order_description_with_contract = Some(order_description_with_contract);
    self
  }

  pub fn order_description_with_contract(&self) -> Option<&String> {
    self.order_description_with_contract.as_ref()
  }

  pub fn reset_order_description_with_contract(&mut self) {
    self.order_description_with_contract = None;
  }

  pub fn set_order_id(&mut self, order_id: i32) {
    self.order_id = Some(order_id);
  }

  pub fn with_order_id(mut self, order_id: i32) -> OrderStatus {
    self.order_id = Some(order_id);
    self
  }

  pub fn order_id(&self) -> Option<&i32> {
    self.order_id.as_ref()
  }

  pub fn reset_order_id(&mut self) {
    self.order_id = None;
  }

  pub fn set_order_not_editable(&mut self, order_not_editable: bool) {
    self.order_not_editable = Some(order_not_editable);
  }

  pub fn with_order_not_editable(mut self, order_not_editable: bool) -> OrderStatus {
    self.order_not_editable = Some(order_not_editable);
    self
  }

  pub fn order_not_editable(&self) -> Option<&bool> {
    self.order_not_editable.as_ref()
  }

  pub fn reset_order_not_editable(&mut self) {
    self.order_not_editable = None;
  }

  pub fn set_order_status(&mut self, order_status: String) {
    self.order_status = Some(order_status);
  }

  pub fn with_order_status(mut self, order_status: String) -> OrderStatus {
    self.order_status = Some(order_status);
    self
  }

  pub fn order_status(&self) -> Option<&String> {
    self.order_status.as_ref()
  }

  pub fn reset_order_status(&mut self) {
    self.order_status = None;
  }

  pub fn set_order_status_description(&mut self, order_status_description: String) {
    self.order_status_description = Some(order_status_description);
  }

  pub fn with_order_status_description(mut self, order_status_description: String) -> OrderStatus {
    self.order_status_description = Some(order_status_description);
    self
  }

  pub fn order_status_description(&self) -> Option<&String> {
    self.order_status_description.as_ref()
  }

  pub fn reset_order_status_description(&mut self) {
    self.order_status_description = None;
  }

  pub fn set_order_time(&mut self, order_time: String) {
    self.order_time = Some(order_time);
  }

  pub fn with_order_time(mut self, order_time: String) -> OrderStatus {
    self.order_time = Some(order_time);
    self
  }

  pub fn order_time(&self) -> Option<&String> {
    self.order_time.as_ref()
  }

  pub fn reset_order_time(&mut self) {
    self.order_time = None;
  }

  pub fn set_order_type(&mut self, order_type: String) {
    self.order_type = Some(order_type);
  }

  pub fn with_order_type(mut self, order_type: String) -> OrderStatus {
    self.order_type = Some(order_type);
    self
  }

  pub fn order_type(&self) -> Option<&String> {
    self.order_type.as_ref()
  }

  pub fn reset_order_type(&mut self) {
    self.order_type = None;
  }

  pub fn set_outside_rth(&mut self, outside_rth: bool) {
    self.outside_rth = Some(outside_rth);
  }

  pub fn with_outside_rth(mut self, outside_rth: bool) -> OrderStatus {
    self.outside_rth = Some(outside_rth);
    self
  }

  pub fn outside_rth(&self) -> Option<&bool> {
    self.outside_rth.as_ref()
  }

  pub fn reset_outside_rth(&mut self) {
    self.outside_rth = None;
  }

  pub fn set_request_id(&mut self, request_id: String) {
    self.request_id = Some(request_id);
  }

  pub fn with_request_id(mut self, request_id: String) -> OrderStatus {
    self.request_id = Some(request_id);
    self
  }

  pub fn request_id(&self) -> Option<&String> {
    self.request_id.as_ref()
  }

  pub fn reset_request_id(&mut self) {
    self.request_id = None;
  }

  pub fn set_sec_type(&mut self, sec_type: String) {
    self.sec_type = Some(sec_type);
  }

  pub fn with_sec_type(mut self, sec_type: String) -> OrderStatus {
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

  pub fn with_side(mut self, side: String) -> OrderStatus {
    self.side = Some(side);
    self
  }

  pub fn side(&self) -> Option<&String> {
    self.side.as_ref()
  }

  pub fn reset_side(&mut self) {
    self.side = None;
  }

  pub fn set_size(&mut self, size: String) {
    self.size = Some(size);
  }

  pub fn with_size(mut self, size: String) -> OrderStatus {
    self.size = Some(size);
    self
  }

  pub fn size(&self) -> Option<&String> {
    self.size.as_ref()
  }

  pub fn reset_size(&mut self) {
    self.size = None;
  }

  pub fn set_size_and_fills(&mut self, size_and_fills: String) {
    self.size_and_fills = Some(size_and_fills);
  }

  pub fn with_size_and_fills(mut self, size_and_fills: String) -> OrderStatus {
    self.size_and_fills = Some(size_and_fills);
    self
  }

  pub fn size_and_fills(&self) -> Option<&String> {
    self.size_and_fills.as_ref()
  }

  pub fn reset_size_and_fills(&mut self) {
    self.size_and_fills = None;
  }

  pub fn set_stop_price(&mut self, stop_price: String) {
    self.stop_price = Some(stop_price);
  }

  pub fn with_stop_price(mut self, stop_price: String) -> OrderStatus {
    self.stop_price = Some(stop_price);
    self
  }

  pub fn stop_price(&self) -> Option<&String> {
    self.stop_price.as_ref()
  }

  pub fn reset_stop_price(&mut self) {
    self.stop_price = None;
  }

  pub fn set_sub_type(&mut self, sub_type: String) {
    self.sub_type = Some(sub_type);
  }

  pub fn with_sub_type(mut self, sub_type: String) -> OrderStatus {
    self.sub_type = Some(sub_type);
    self
  }

  pub fn sub_type(&self) -> Option<&String> {
    self.sub_type.as_ref()
  }

  pub fn reset_sub_type(&mut self) {
    self.sub_type = None;
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: String) -> OrderStatus {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

  pub fn set_tif(&mut self, tif: String) {
    self.tif = Some(tif);
  }

  pub fn with_tif(mut self, tif: String) -> OrderStatus {
    self.tif = Some(tif);
    self
  }

  pub fn tif(&self) -> Option<&String> {
    self.tif.as_ref()
  }

  pub fn reset_tif(&mut self) {
    self.tif = None;
  }

  pub fn set_total_size(&mut self, total_size: String) {
    self.total_size = Some(total_size);
  }

  pub fn with_total_size(mut self, total_size: String) -> OrderStatus {
    self.total_size = Some(total_size);
    self
  }

  pub fn total_size(&self) -> Option<&String> {
    self.total_size.as_ref()
  }

  pub fn reset_total_size(&mut self) {
    self.total_size = None;
  }

  pub fn set_use_price_mgmt_algo(&mut self, use_price_mgmt_algo: bool) {
    self.use_price_mgmt_algo = Some(use_price_mgmt_algo);
  }

  pub fn with_use_price_mgmt_algo(mut self, use_price_mgmt_algo: bool) -> OrderStatus {
    self.use_price_mgmt_algo = Some(use_price_mgmt_algo);
    self
  }

  pub fn use_price_mgmt_algo(&self) -> Option<&bool> {
    self.use_price_mgmt_algo.as_ref()
  }

  pub fn reset_use_price_mgmt_algo(&mut self) {
    self.use_price_mgmt_algo = None;
  }

}




/* 
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SecdefInfo : Contains some basic info of contract

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecdefInfo {
  #[serde(rename = "conid")]
  conid: Option<f32>,
  /// Annual interest rate paid on a bond
  #[serde(rename = "coupon")]
  coupon: Option<String>,
  /// Currency the contract trades in
  #[serde(rename = "currency")]
  currency: Option<String>,
  /// Committee on Uniform Securities Identification Procedures number
  #[serde(rename = "cusip")]
  cusip: Option<String>,
  /// Formatted symbol
  #[serde(rename = "desc1")]
  desc1: Option<String>,
  /// Formatted expiration, strike and right
  #[serde(rename = "desc2")]
  desc2: Option<String>,
  #[serde(rename = "exchange")]
  exchange: Option<String>,
  #[serde(rename = "listingExchange")]
  listing_exchange: Option<String>,
  /// Format YYYYMMDD, the date on which the underlying transaction settles if the option is exercised
  #[serde(rename = "maturityDate")]
  maturity_date: Option<String>,
  /// total premium paid or received for an option contract
  #[serde(rename = "multiplier")]
  multiplier: Option<String>,
  /// C = Call Option, P = Put Option
  #[serde(rename = "right")]
  right: Option<String>,
  #[serde(rename = "secType")]
  sec_type: Option<String>,
  /// The strike price also known as exercise price
  #[serde(rename = "strike")]
  strike: Option<String>,
  /// Underlying symbol
  #[serde(rename = "symbol")]
  symbol: Option<String>,
  #[serde(rename = "tradingClass")]
  trading_class: Option<String>,
  #[serde(rename = "validExchanges")]
  valid_exchanges: Option<String>
}

impl SecdefInfo {
  /// Contains some basic info of contract
  pub fn new() -> SecdefInfo {
    SecdefInfo {
      conid: None,
      coupon: None,
      currency: None,
      cusip: None,
      desc1: None,
      desc2: None,
      exchange: None,
      listing_exchange: None,
      maturity_date: None,
      multiplier: None,
      right: None,
      sec_type: None,
      strike: None,
      symbol: None,
      trading_class: None,
      valid_exchanges: None
    }
  }

  pub fn set_conid(&mut self, conid: f32) {
    self.conid = Some(conid);
  }

  pub fn with_conid(mut self, conid: f32) -> SecdefInfo {
    self.conid = Some(conid);
    self
  }

  pub fn conid(&self) -> Option<&f32> {
    self.conid.as_ref()
  }

  pub fn reset_conid(&mut self) {
    self.conid = None;
  }

  pub fn set_coupon(&mut self, coupon: String) {
    self.coupon = Some(coupon);
  }

  pub fn with_coupon(mut self, coupon: String) -> SecdefInfo {
    self.coupon = Some(coupon);
    self
  }

  pub fn coupon(&self) -> Option<&String> {
    self.coupon.as_ref()
  }

  pub fn reset_coupon(&mut self) {
    self.coupon = None;
  }

  pub fn set_currency(&mut self, currency: String) {
    self.currency = Some(currency);
  }

  pub fn with_currency(mut self, currency: String) -> SecdefInfo {
    self.currency = Some(currency);
    self
  }

  pub fn currency(&self) -> Option<&String> {
    self.currency.as_ref()
  }

  pub fn reset_currency(&mut self) {
    self.currency = None;
  }

  pub fn set_cusip(&mut self, cusip: String) {
    self.cusip = Some(cusip);
  }

  pub fn with_cusip(mut self, cusip: String) -> SecdefInfo {
    self.cusip = Some(cusip);
    self
  }

  pub fn cusip(&self) -> Option<&String> {
    self.cusip.as_ref()
  }

  pub fn reset_cusip(&mut self) {
    self.cusip = None;
  }

  pub fn set_desc1(&mut self, desc1: String) {
    self.desc1 = Some(desc1);
  }

  pub fn with_desc1(mut self, desc1: String) -> SecdefInfo {
    self.desc1 = Some(desc1);
    self
  }

  pub fn desc1(&self) -> Option<&String> {
    self.desc1.as_ref()
  }

  pub fn reset_desc1(&mut self) {
    self.desc1 = None;
  }

  pub fn set_desc2(&mut self, desc2: String) {
    self.desc2 = Some(desc2);
  }

  pub fn with_desc2(mut self, desc2: String) -> SecdefInfo {
    self.desc2 = Some(desc2);
    self
  }

  pub fn desc2(&self) -> Option<&String> {
    self.desc2.as_ref()
  }

  pub fn reset_desc2(&mut self) {
    self.desc2 = None;
  }

  pub fn set_exchange(&mut self, exchange: String) {
    self.exchange = Some(exchange);
  }

  pub fn with_exchange(mut self, exchange: String) -> SecdefInfo {
    self.exchange = Some(exchange);
    self
  }

  pub fn exchange(&self) -> Option<&String> {
    self.exchange.as_ref()
  }

  pub fn reset_exchange(&mut self) {
    self.exchange = None;
  }

  pub fn set_listing_exchange(&mut self, listing_exchange: String) {
    self.listing_exchange = Some(listing_exchange);
  }

  pub fn with_listing_exchange(mut self, listing_exchange: String) -> SecdefInfo {
    self.listing_exchange = Some(listing_exchange);
    self
  }

  pub fn listing_exchange(&self) -> Option<&String> {
    self.listing_exchange.as_ref()
  }

  pub fn reset_listing_exchange(&mut self) {
    self.listing_exchange = None;
  }

  pub fn set_maturity_date(&mut self, maturity_date: String) {
    self.maturity_date = Some(maturity_date);
  }

  pub fn with_maturity_date(mut self, maturity_date: String) -> SecdefInfo {
    self.maturity_date = Some(maturity_date);
    self
  }

  pub fn maturity_date(&self) -> Option<&String> {
    self.maturity_date.as_ref()
  }

  pub fn reset_maturity_date(&mut self) {
    self.maturity_date = None;
  }

  pub fn set_multiplier(&mut self, multiplier: String) {
    self.multiplier = Some(multiplier);
  }

  pub fn with_multiplier(mut self, multiplier: String) -> SecdefInfo {
    self.multiplier = Some(multiplier);
    self
  }

  pub fn multiplier(&self) -> Option<&String> {
    self.multiplier.as_ref()
  }

  pub fn reset_multiplier(&mut self) {
    self.multiplier = None;
  }

  pub fn set_right(&mut self, right: String) {
    self.right = Some(right);
  }

  pub fn with_right(mut self, right: String) -> SecdefInfo {
    self.right = Some(right);
    self
  }

  pub fn right(&self) -> Option<&String> {
    self.right.as_ref()
  }

  pub fn reset_right(&mut self) {
    self.right = None;
  }

  pub fn set_sec_type(&mut self, sec_type: String) {
    self.sec_type = Some(sec_type);
  }

  pub fn with_sec_type(mut self, sec_type: String) -> SecdefInfo {
    self.sec_type = Some(sec_type);
    self
  }

  pub fn sec_type(&self) -> Option<&String> {
    self.sec_type.as_ref()
  }

  pub fn reset_sec_type(&mut self) {
    self.sec_type = None;
  }

  pub fn set_strike(&mut self, strike: String) {
    self.strike = Some(strike);
  }

  pub fn with_strike(mut self, strike: String) -> SecdefInfo {
    self.strike = Some(strike);
    self
  }

  pub fn strike(&self) -> Option<&String> {
    self.strike.as_ref()
  }

  pub fn reset_strike(&mut self) {
    self.strike = None;
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: String) -> SecdefInfo {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

  pub fn set_trading_class(&mut self, trading_class: String) {
    self.trading_class = Some(trading_class);
  }

  pub fn with_trading_class(mut self, trading_class: String) -> SecdefInfo {
    self.trading_class = Some(trading_class);
    self
  }

  pub fn trading_class(&self) -> Option<&String> {
    self.trading_class.as_ref()
  }

  pub fn reset_trading_class(&mut self) {
    self.trading_class = None;
  }

  pub fn set_valid_exchanges(&mut self, valid_exchanges: String) {
    self.valid_exchanges = Some(valid_exchanges);
  }

  pub fn with_valid_exchanges(mut self, valid_exchanges: String) -> SecdefInfo {
    self.valid_exchanges = Some(valid_exchanges);
    self
  }

  pub fn valid_exchanges(&self) -> Option<&String> {
    self.valid_exchanges.as_ref()
  }

  pub fn reset_valid_exchanges(&mut self) {
    self.valid_exchanges = None;
  }

}




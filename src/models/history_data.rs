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
pub struct HistoryData {
  /// The number of seconds in a bar
  #[serde(rename = "barLength")]
  bar_length: Option<i32>,
  #[serde(rename = "data")]
  data: Option<Vec<::models::HistorydataData>>,
  /// High value during this time series with format %h/%v/%t. %h is the high price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume = actual volume/100)) and %t is minutes from start time of the chart 
  #[serde(rename = "high")]
  high: Option<String>,
  /// Low value during this time series with format %l/%v/%t. %l is the low price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume = actual volume/100)) and %t is minutes from start time of the chart 
  #[serde(rename = "low")]
  low: Option<String>,
  /// Market Data Availability. The field may contain two chars. The first char is the primary code: S = Streaming, R = Realtime, D = Delayed, Z = Frozen, Y = Frozen Delayed. The second char is the secondary code: P = Snapshot Available, p = Consolidated. 
  #[serde(rename = "mdAvailability")]
  md_availability: Option<String>,
  #[serde(rename = "messageVersion")]
  message_version: Option<i32>,
  /// The time it takes, in milliseconds, to process the historical data request
  #[serde(rename = "mktDataDelay")]
  mkt_data_delay: Option<i32>,
  #[serde(rename = "negativeCapable")]
  negative_capable: Option<bool>,
  /// The historical data returned includes outside of regular trading hours 
  #[serde(rename = "outsideRth")]
  outside_rth: Option<bool>,
  /// total number of points
  #[serde(rename = "points")]
  points: Option<i32>,
  #[serde(rename = "priceDisplayRule")]
  price_display_rule: Option<i32>,
  #[serde(rename = "priceDisplayValue")]
  price_display_value: Option<String>,
  /// priceFactor is price increment obtained from display rule
  #[serde(rename = "priceFactor")]
  price_factor: Option<i32>,
  /// start date time in the format YYYYMMDD-HH:mm:ss
  #[serde(rename = "startTime")]
  start_time: Option<String>,
  /// Underlying symbol
  #[serde(rename = "symbol")]
  symbol: Option<String>,
  /// companyName
  #[serde(rename = "text")]
  text: Option<String>,
  /// The duration for the historical data request
  #[serde(rename = "timePeriod")]
  time_period: Option<String>,
  /// The number of seconds in the trading day
  #[serde(rename = "tradingDayDuration")]
  trading_day_duration: Option<i32>,
  #[serde(rename = "travelTime")]
  travel_time: Option<i32>,
  #[serde(rename = "volumeFactor")]
  volume_factor: Option<i32>
}

impl HistoryData {
  pub fn new() -> HistoryData {
    HistoryData {
      bar_length: None,
      data: None,
      high: None,
      low: None,
      md_availability: None,
      message_version: None,
      mkt_data_delay: None,
      negative_capable: None,
      outside_rth: None,
      points: None,
      price_display_rule: None,
      price_display_value: None,
      price_factor: None,
      start_time: None,
      symbol: None,
      text: None,
      time_period: None,
      trading_day_duration: None,
      travel_time: None,
      volume_factor: None
    }
  }

  pub fn set_bar_length(&mut self, bar_length: i32) {
    self.bar_length = Some(bar_length);
  }

  pub fn with_bar_length(mut self, bar_length: i32) -> HistoryData {
    self.bar_length = Some(bar_length);
    self
  }

  pub fn bar_length(&self) -> Option<&i32> {
    self.bar_length.as_ref()
  }

  pub fn reset_bar_length(&mut self) {
    self.bar_length = None;
  }

  pub fn set_data(&mut self, data: Vec<::models::HistorydataData>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: Vec<::models::HistorydataData>) -> HistoryData {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&Vec<::models::HistorydataData>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_high(&mut self, high: String) {
    self.high = Some(high);
  }

  pub fn with_high(mut self, high: String) -> HistoryData {
    self.high = Some(high);
    self
  }

  pub fn high(&self) -> Option<&String> {
    self.high.as_ref()
  }

  pub fn reset_high(&mut self) {
    self.high = None;
  }

  pub fn set_low(&mut self, low: String) {
    self.low = Some(low);
  }

  pub fn with_low(mut self, low: String) -> HistoryData {
    self.low = Some(low);
    self
  }

  pub fn low(&self) -> Option<&String> {
    self.low.as_ref()
  }

  pub fn reset_low(&mut self) {
    self.low = None;
  }

  pub fn set_md_availability(&mut self, md_availability: String) {
    self.md_availability = Some(md_availability);
  }

  pub fn with_md_availability(mut self, md_availability: String) -> HistoryData {
    self.md_availability = Some(md_availability);
    self
  }

  pub fn md_availability(&self) -> Option<&String> {
    self.md_availability.as_ref()
  }

  pub fn reset_md_availability(&mut self) {
    self.md_availability = None;
  }

  pub fn set_message_version(&mut self, message_version: i32) {
    self.message_version = Some(message_version);
  }

  pub fn with_message_version(mut self, message_version: i32) -> HistoryData {
    self.message_version = Some(message_version);
    self
  }

  pub fn message_version(&self) -> Option<&i32> {
    self.message_version.as_ref()
  }

  pub fn reset_message_version(&mut self) {
    self.message_version = None;
  }

  pub fn set_mkt_data_delay(&mut self, mkt_data_delay: i32) {
    self.mkt_data_delay = Some(mkt_data_delay);
  }

  pub fn with_mkt_data_delay(mut self, mkt_data_delay: i32) -> HistoryData {
    self.mkt_data_delay = Some(mkt_data_delay);
    self
  }

  pub fn mkt_data_delay(&self) -> Option<&i32> {
    self.mkt_data_delay.as_ref()
  }

  pub fn reset_mkt_data_delay(&mut self) {
    self.mkt_data_delay = None;
  }

  pub fn set_negative_capable(&mut self, negative_capable: bool) {
    self.negative_capable = Some(negative_capable);
  }

  pub fn with_negative_capable(mut self, negative_capable: bool) -> HistoryData {
    self.negative_capable = Some(negative_capable);
    self
  }

  pub fn negative_capable(&self) -> Option<&bool> {
    self.negative_capable.as_ref()
  }

  pub fn reset_negative_capable(&mut self) {
    self.negative_capable = None;
  }

  pub fn set_outside_rth(&mut self, outside_rth: bool) {
    self.outside_rth = Some(outside_rth);
  }

  pub fn with_outside_rth(mut self, outside_rth: bool) -> HistoryData {
    self.outside_rth = Some(outside_rth);
    self
  }

  pub fn outside_rth(&self) -> Option<&bool> {
    self.outside_rth.as_ref()
  }

  pub fn reset_outside_rth(&mut self) {
    self.outside_rth = None;
  }

  pub fn set_points(&mut self, points: i32) {
    self.points = Some(points);
  }

  pub fn with_points(mut self, points: i32) -> HistoryData {
    self.points = Some(points);
    self
  }

  pub fn points(&self) -> Option<&i32> {
    self.points.as_ref()
  }

  pub fn reset_points(&mut self) {
    self.points = None;
  }

  pub fn set_price_display_rule(&mut self, price_display_rule: i32) {
    self.price_display_rule = Some(price_display_rule);
  }

  pub fn with_price_display_rule(mut self, price_display_rule: i32) -> HistoryData {
    self.price_display_rule = Some(price_display_rule);
    self
  }

  pub fn price_display_rule(&self) -> Option<&i32> {
    self.price_display_rule.as_ref()
  }

  pub fn reset_price_display_rule(&mut self) {
    self.price_display_rule = None;
  }

  pub fn set_price_display_value(&mut self, price_display_value: String) {
    self.price_display_value = Some(price_display_value);
  }

  pub fn with_price_display_value(mut self, price_display_value: String) -> HistoryData {
    self.price_display_value = Some(price_display_value);
    self
  }

  pub fn price_display_value(&self) -> Option<&String> {
    self.price_display_value.as_ref()
  }

  pub fn reset_price_display_value(&mut self) {
    self.price_display_value = None;
  }

  pub fn set_price_factor(&mut self, price_factor: i32) {
    self.price_factor = Some(price_factor);
  }

  pub fn with_price_factor(mut self, price_factor: i32) -> HistoryData {
    self.price_factor = Some(price_factor);
    self
  }

  pub fn price_factor(&self) -> Option<&i32> {
    self.price_factor.as_ref()
  }

  pub fn reset_price_factor(&mut self) {
    self.price_factor = None;
  }

  pub fn set_start_time(&mut self, start_time: String) {
    self.start_time = Some(start_time);
  }

  pub fn with_start_time(mut self, start_time: String) -> HistoryData {
    self.start_time = Some(start_time);
    self
  }

  pub fn start_time(&self) -> Option<&String> {
    self.start_time.as_ref()
  }

  pub fn reset_start_time(&mut self) {
    self.start_time = None;
  }

  pub fn set_symbol(&mut self, symbol: String) {
    self.symbol = Some(symbol);
  }

  pub fn with_symbol(mut self, symbol: String) -> HistoryData {
    self.symbol = Some(symbol);
    self
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn reset_symbol(&mut self) {
    self.symbol = None;
  }

  pub fn set_text(&mut self, text: String) {
    self.text = Some(text);
  }

  pub fn with_text(mut self, text: String) -> HistoryData {
    self.text = Some(text);
    self
  }

  pub fn text(&self) -> Option<&String> {
    self.text.as_ref()
  }

  pub fn reset_text(&mut self) {
    self.text = None;
  }

  pub fn set_time_period(&mut self, time_period: String) {
    self.time_period = Some(time_period);
  }

  pub fn with_time_period(mut self, time_period: String) -> HistoryData {
    self.time_period = Some(time_period);
    self
  }

  pub fn time_period(&self) -> Option<&String> {
    self.time_period.as_ref()
  }

  pub fn reset_time_period(&mut self) {
    self.time_period = None;
  }

  pub fn set_trading_day_duration(&mut self, trading_day_duration: i32) {
    self.trading_day_duration = Some(trading_day_duration);
  }

  pub fn with_trading_day_duration(mut self, trading_day_duration: i32) -> HistoryData {
    self.trading_day_duration = Some(trading_day_duration);
    self
  }

  pub fn trading_day_duration(&self) -> Option<&i32> {
    self.trading_day_duration.as_ref()
  }

  pub fn reset_trading_day_duration(&mut self) {
    self.trading_day_duration = None;
  }

  pub fn set_travel_time(&mut self, travel_time: i32) {
    self.travel_time = Some(travel_time);
  }

  pub fn with_travel_time(mut self, travel_time: i32) -> HistoryData {
    self.travel_time = Some(travel_time);
    self
  }

  pub fn travel_time(&self) -> Option<&i32> {
    self.travel_time.as_ref()
  }

  pub fn reset_travel_time(&mut self) {
    self.travel_time = None;
  }

  pub fn set_volume_factor(&mut self, volume_factor: i32) {
    self.volume_factor = Some(volume_factor);
  }

  pub fn with_volume_factor(mut self, volume_factor: i32) -> HistoryData {
    self.volume_factor = Some(volume_factor);
    self
  }

  pub fn volume_factor(&self) -> Option<&i32> {
    self.volume_factor.as_ref()
  }

  pub fn reset_volume_factor(&mut self) {
    self.volume_factor = None;
  }

}




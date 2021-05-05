use core::f64;
use chrono::{DateTime, Utc};
fn main() {
    println!("SigFox Message Parser");
    let dp = DevicePayload { payload: String::from("424511121f001082087e1412")};
    let pl = dp.parse_data();
}

// For info on SigFox message Metadata  See: https://support.sigfox.com/docs/data-advanced
// For this use case I will only use data, device_id, lqi, Operator and Timestamp
struct SigFoxMsg {
    radius: f64, // Radius (Computed location)
    longitude: f64, // Longitude (Computed location)
    latitude: f64, // Latitude (Computed location)
    source: f64, // Source (Computed location)
    country: String,
    data: String, // Hexadecimal. Example: 424511121f001082087e1412
    device_id: String, // Max 8 Hexadecimal characters. Example: 002C37D2
    lqi: String, // Link Quality Indicator. Example: Good
    rssi: String, // (RSSI rather than SNR) Link Quality Indicator. Example: Good
    lqir: String, // Link Quality Indicator repeaters
    operator: String, // SigFox Example: SIGFOX_NewZealand_Thinxtra
    timestamp: DateTime<Utc> // Timestamp. Example: 2020-04-30 10:39:29
}
#[warn(unused_imports)]
struct DevicePayload {
    payload: String
}
impl Parse for DevicePayload {
    fn parse_data(&self) -> Self {
        DevicePayload {payload: format!("{}", self.payload) } 
    }
}

trait Parse {
    fn parse_data(&self) -> Self;
}
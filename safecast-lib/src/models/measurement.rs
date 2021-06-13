pub struct Measurement {
    pub id: Option<i64>,
    pub captured_at: Option<String>,
    pub channel_id: Option<i64>,
    pub device_id: Option<i64>,
    pub devicetype_id: Option<i64>,
    pub height: Option<i64>,
    pub latitude: Option<f64>,
    pub location_name: Option<String>,
    pub longitude: Option<f64>,
    pub measurement_import_id: Option<i64>,
    pub original_id: Option<i64>,
    pub sensor_id: Option<i64>,
    pub station_id: Option<i64>,
    pub unit: String,
    pub user_id: Option<i64>,
    pub value: i64,
}

impl Measurement {
    pub fn new(unit: String, value: i64) -> Self {
        Measurement {
            id: None,
            captured_at: None,
            channel_id: None,
            device_id: None,
            devicetype_id: None,
            height: None,
            latitude: None,
            location_name: None,
            longitude: None,
            measurement_import_id: None,
            original_id: None,
            sensor_id: None,
            station_id: None,
            unit,
            user_id: None,
            value,
        }
    }
}

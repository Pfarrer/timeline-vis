use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Location {
    #[serde(rename = "timestampMs")]
    pub timestamp_ms: String,
    #[serde(rename = "latitudeE7")]
    pub latitude_e7: u32,
    #[serde(rename = "longitudeE7")]
    pub longitude_e7: u32,
    pub accuracy: i32,
    pub activities: Option<Vec<Classification>>,
}

#[derive(Debug, Deserialize)]
pub struct Classification {
    #[serde(rename = "timestampMs")]
    pub timestamp_ms: String,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize)]
pub struct Activity {
    pub confidence: u32,
    pub activity: ActivityType,
}

#[derive(Debug, Deserialize)]
pub enum ActivityType {
    Unknown(u8),
    InVehicle(u8),
    OnBicycle(u8),
    OnFoot(u8),
    Walking(u8),
    Running(u8),
    Still(u8),
    InRoadVehicle(u8),
    InRailVehicle(u8),
    InFourWheelerVehicle(u8),
    InCar(u8),
}

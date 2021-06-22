use serde::Deserialize;
use serde_json;

pub fn parse_semantic(json_string: &str) -> Wrapper {
    match serde_json::from_str(json_string) {
        Ok(it) => it,
        Err(e) => panic!("Deserialization failed:\n{}\n{}", e, json_string),
    }
}

pub fn parse_raw_location(json_string: &str) -> RawLocation {
    match serde_json::from_str(json_string) {
        Ok(it) => it,
        Err(e) => panic!("Deserialization failed:\n{}\n{}", e, json_string),
    }
}

#[derive(Deserialize)]
pub struct Wrapper {
    #[serde(rename = "activitySegment")]
    pub activity_segment: Option<ActivitySegment>,

    #[serde(rename = "placeVisit")]
    pub place_visit: Option<PlaceVisit>,
}

#[derive(Deserialize)]
pub struct ActivitySegment {
    pub duration: Duration,

    pub distance: Option<i32>,

    #[serde(rename = "startLocation")]
    pub start_location: Location,

    #[serde(rename = "endLocation")]
    pub end_location: Location,

    pub activities: Vec<Activity>,

    #[serde(rename = "activityType")]
    pub activity_type: String,

    #[serde(rename = "waypointPath")]
    pub waypoint_path: Option<WaypointPath>,
}

#[derive(Deserialize)]
pub struct PlaceVisit {}

#[derive(Deserialize)]
pub struct Location {
    #[serde(rename = "latitudeE7")]
    pub latitude_e7: Option<i64>,

    #[serde(rename = "longitudeE7")]
    pub longitude_e7: Option<i64>,

    pub address: Option<String>,

    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct Duration {
    #[serde(rename = "startTimestampMs")]
    pub start_timestamp_ms_string: String,

    #[serde(rename = "endTimestampMs")]
    pub end_timestamp_ms_string: String,
}

#[derive(Deserialize)]
pub struct Activity {
    #[serde(rename = "activityType")]
    activity_type: String,

    probability: f64,
}

#[derive(Deserialize)]
pub struct WaypointPath {
    pub waypoints: Vec<Waypoint>,
}

#[derive(Deserialize)]
pub struct Waypoint {
    #[serde(rename = "latE7")]
    pub lat_e7: i64,

    #[serde(rename = "lngE7")]
    pub lng_e7: i64,
}

#[derive(Deserialize)]
pub struct RawLocation {
    #[serde(rename = "timestampMs")]
    pub timestamp_ms: String,

    #[serde(rename = "latitudeE7")]
    pub latitude_e7: Option<i64>,

    #[serde(rename = "longitudeE7")]
    pub longitude_e7: Option<i64>,
}

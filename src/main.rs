use serde::{Deserialize, Serialize};
use std::io;
use std::io::BufRead;

fn main() {
    for line in io::stdin().lock().lines() {
        match parse_line(line.unwrap()) {
            Ok(record) => {
                if let Some(activity_segment) = record.activity_segment {
                    let (bulk_info, converted) = convert_activity_segment(activity_segment);
                    println!("{}", serde_json::to_string(&bulk_info).unwrap());
                    println!("{}", serde_json::to_string(&converted).unwrap());
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn parse_line(line: String) -> Result<RecordWrapper, serde_json::error::Error> {
    return serde_json::from_str(&line);
}

fn convert_activity_segment(original: ActivitySegment) -> (BulkInfo, ActivitySegment) {
    let duration = original.duration.es_convert();
    let start_location = original.start_location.es_convert();
    let end_location = original.end_location.es_convert();
    let waypoints = Some(original.es_convert());

    let updated = ActivitySegment {
        duration,
        start_location,
        end_location,
        waypoints,
        ..original
    };

    let index_info = IndexInfo {
        index: "activitysegment".into(),
        id: format!("{}", updated.duration.start_timestamp_ms_string),
    };

    let bulk_info = BulkInfo { index: index_info };

    (bulk_info, updated)
}

#[derive(Serialize)]
struct BulkInfo {
    index: IndexInfo,
}

#[derive(Serialize)]
struct IndexInfo {
    #[serde(rename = "_index")]
    index: String,

    #[serde(rename = "_id")]
    id: String,
}

#[derive(Serialize, Deserialize)]
struct RecordWrapper {
    #[serde(rename = "activitySegment")]
    activity_segment: Option<ActivitySegment>,

    #[serde(rename = "placeVisit")]
    place_visit: Option<PlaceVisit>,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
struct ActivitySegment {
    duration: Duration,

    #[serde(rename = "startLocation")]
    start_location: Location,

    #[serde(rename = "endLocation")]
    end_location: Location,

    activities: Vec<Activity>,

    #[serde(rename = "waypointPath", skip_serializing)]
    waypoint_path: WaypointPath,

    waypoints: Option<EsLineString>,
}

#[derive(Serialize, Deserialize)]
struct PlaceVisit {}

#[derive(Serialize, Deserialize, Clone)]
#[allow(dead_code)]
struct Location {
    #[serde(rename = "latitudeE7", skip_serializing)]
    latitude_e7: u64,

    #[serde(rename = "longitudeE7", skip_serializing)]
    longitude_e7: u64,

    #[serde(rename = "geoLocation")]
    es_location: Option<EsLocation>,

    address: Option<String>,

    name: Option<String>,

    #[serde(rename = "semanticType")]
    semantic_type: Option<String>,

    #[serde(rename = "locationConfidence")]
    location_confidence: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Duration {
    #[serde(rename = "startTimestampMs", skip_serializing)]
    start_timestamp_ms_string: String,

    #[serde(rename = "endTimestampMs", skip_serializing)]
    end_timestamp_ms_string: String,

    #[serde(rename = "startTimestampMs", skip_deserializing)]
    start_timestamp_ms_u64: Option<u64>,

    #[serde(rename = "endTimestampMs", skip_deserializing)]
    end_timestamp_ms_u64: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
struct Activity {
    #[serde(rename = "activityType")]
    activity_type: String,

    probability: f64,
}

#[derive(Serialize, Deserialize)]
struct WaypointPath {
    waypoints: Vec<Waypoint>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Waypoint {
    #[serde(rename = "latE7", skip_serializing)]
    lat_e7: u64,

    #[serde(rename = "lngE7", skip_serializing)]
    lng_e7: u64,

    #[serde(rename = "geoLocation")]
    es_location: Option<EsLocation>,
}

#[derive(Serialize, Deserialize, Clone)]
struct EsLocation {
    lat: f64,
    lon: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct EsLineString {
    #[serde(rename = "type")]
    es_type: String,

    coordinates: Vec<Vec<f64>>,
}

trait EsConvertable<T> {
    fn es_convert(&self) -> T;
}

impl EsConvertable<Location> for Location {
    fn es_convert(&self) -> Location {
        let es_location = Some(EsLocation {
            lat: self.latitude_e7 as f64 / 1e7,
            lon: self.longitude_e7 as f64 / 1e7,
        });

        Location {
            es_location,
            ..(*self).clone()
        }
    }
}

impl EsConvertable<EsLineString> for ActivitySegment {
    fn es_convert(&self) -> EsLineString {
        let coordinates = self
            .waypoint_path
            .waypoints
            .iter()
            .map(|waypoint| vec![waypoint.lng_e7 as f64 / 1e7, waypoint.lat_e7 as f64 / 1e7])
            .collect();

        EsLineString {
            es_type: "linestring".into(),
            coordinates,
        }
    }
}

impl EsConvertable<Duration> for Duration {
    fn es_convert(&self) -> Duration {
        let start_timestamp_ms_u64 = Some(self.start_timestamp_ms_string.parse::<u64>().unwrap());
        let end_timestamp_ms_u64 = Some(self.end_timestamp_ms_string.parse::<u64>().unwrap());

        Duration {
            start_timestamp_ms_u64,
            end_timestamp_ms_u64,
            ..(*self).clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_segment() {
        let json_string = r#"{"activitySegment":{"startLocation":{"latitudeE7":498746569,"longitudeE7":86486560,"placeId":"ChIJvdwoQGJwvUcRGF9pJrg4Px8","address":"S4|02\nGrafenstraße 2\n64283 Darmstadt\nDeutschland","name":"S4|02","semanticType":"TYPE_WORK","locationConfidence":41.26615},"endLocation":{"latitudeE7":498743210,"longitudeE7":86577130,"placeId":"ChIJFer2NWRwvUcRChISFK9g3cI","address":"Schloßgraben 1\n64283 Darmstadt\nDeutschland","name":"Darmstadtium - Science and Congress Center","locationConfidence":38.52593},"duration":{"startTimestampMs":"1354530532487","endTimestampMs":"1354530712585"},"distance":372,"confidence":"MEDIUM","activities":[{"activityType":"WALKING","probability":0},{"activityType":"CYCLING","probability":0},{"activityType":"IN_VEHICLE","probability":0},{"activityType":"FLYING","probability":0}],"waypointPath":{"waypoints":[{"latE7":498776969,"lngE7":86550121},{"latE7":498756523,"lngE7":86569156}]},"editConfirmationStatus":"NOT_CONFIRMED"}}"#;
        let record: RecordWrapper = parse_line(json_string.into()).unwrap();

        assert!(record.activity_segment.is_some());
        assert!(record.place_visit.is_none());

        let activity_segment = record.activity_segment.unwrap();
        assert_eq!(activity_segment.activities.len(), 4);
        assert_eq!(activity_segment.waypoint_path.waypoints.len(), 2);
    }
}

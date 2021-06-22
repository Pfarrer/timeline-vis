use serde::Serialize;

use crate::google;

pub fn map_activity_segement(
    google_activity_segment: google::ActivitySegment,
    raw_locations: &Vec<google::RawLocation>,
) -> ActivitySegment {
    let start_timestamp = google_activity_segment
        .duration
        .start_timestamp_ms_string
        .parse()
        .unwrap();
    let end_timestamp = google_activity_segment
        .duration
        .end_timestamp_ms_string
        .parse()
        .unwrap();
    let distance = google_activity_segment.distance.unwrap_or(0);
    let average_speed = if distance > 0 {
        let duration_in_hours = (end_timestamp - start_timestamp) as f32 / 1000f32 / 60f32 / 60f32;
        let speed = distance as f32 / 1000f32 / duration_in_hours;
        round_to_decimal_places(speed, 1)
    } else {
        0 as f32
    };

    let raw_coordinates = get_coordinates_between(start_timestamp, end_timestamp, raw_locations);
    let semantic_coordinates = if let Some(path) = google_activity_segment.waypoint_path {
        let start_coordindate = google_activity_segment.start_location.to_coordinates();
        let end_coordindate = google_activity_segment.end_location.to_coordinates();

        let mut waypoints: Vec<Vec<f32>> = path
            .waypoints
            .iter()
            .map(|waypoint| vec![waypoint.lng_e7 as f32 / 1e7, waypoint.lat_e7 as f32 / 1e7])
            .collect();

        let mut coordinates: Vec<Vec<f32>> = vec![];
        if let Some(start) = start_coordindate {
            coordinates.append(&mut vec![start])
        }
        coordinates.append(&mut waypoints);
        if let Some(end) = end_coordindate {
            coordinates.append(&mut vec![end])
        }
        coordinates
    } else {
        vec![]
    };
    let semantic_waypoints = if semantic_coordinates.is_empty() {
        None
    } else {
        Some(GeoLineString::of(semantic_coordinates))
    };
    let raw_waypoints = if raw_coordinates.is_empty() {
        None
    } else {
        Some(GeoLineString::of(raw_coordinates))
    };

    ActivitySegment {
        start_timestamp,
        end_timestamp,
        distance,
        average_speed,
        activity_type: google_activity_segment.activity_type,
        semantic_waypoints,
        raw_waypoints,
    }
}

fn get_coordinates_between(
    start_timestamp: u64,
    end_timestamp: u64,
    raw_locations: &Vec<google::RawLocation>,
) -> Vec<Vec<f32>> {
    raw_locations
        .iter()
        .skip_while(|loc| loc.timestamp_ms.parse::<u64>().unwrap() < start_timestamp)
        .take_while(|loc| loc.timestamp_ms.parse::<u64>().unwrap() <= end_timestamp)
        .filter_map(|loc| {
            if let (Some(lat_e7), Some(lng_e7)) = (loc.latitude_e7, loc.longitude_e7) {
                Some(vec![lng_e7 as f32 / 1e7, lat_e7 as f32 / 1e7])
            } else {
                None
            }
        })
        .collect()
}

impl google::Location {
    fn to_coordinates(&self) -> Option<Vec<f32>> {
        if let (Some(lat_e7), Some(lng_e7)) = (self.latitude_e7, self.longitude_e7) {
            Some(vec![
                google_latlng_to_es_latlng(lng_e7),
                google_latlng_to_es_latlng(lat_e7),
            ])
        } else {
            None
        }
    }
}

fn google_latlng_to_es_latlng(g: i64) -> f32 {
    g as f32 / 1e7
}

fn round_to_decimal_places(num: f32, decimals: u8) -> f32 {
    let shift = 10f32.powf(decimals as f32);
    (num * shift).round() / shift
}

pub fn print_activity_segement(activity_segement: ActivitySegment) {
    println!(
        r#"{{ "index" : {{ "_index" : "activity_segment", "_id" : "{}" }} }}"#,
        activity_segement.start_timestamp
    );
    println!("{}", serde_json::to_string(&activity_segement).unwrap());
}

pub fn map_place_visit(place_visit: google::PlaceVisit) -> PlaceVisit {
    PlaceVisit {}
}

#[derive(Serialize)]
pub struct ActivitySegment {
    start_timestamp: u64,
    end_timestamp: u64,
    distance: i32,
    average_speed: f32,
    activity_type: String,
    semantic_waypoints: Option<GeoLineString>,
    raw_waypoints: Option<GeoLineString>,
}

#[derive(Serialize)]
pub struct PlaceVisit {}

#[derive(Serialize)]
pub struct GeoLineString {
    r#type: String,
    coordinates: Vec<Vec<f32>>,
}

impl GeoLineString {
    pub fn of(coordinates: Vec<Vec<f32>>) -> GeoLineString {
        GeoLineString {
            r#type: "linestring".to_owned(),
            coordinates,
        }
    }
}

use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod es;
mod google;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_locations: Vec<google::RawLocation> = read_lines(args[1].clone())
        .map(|line| google::parse_raw_location(&line.unwrap()))
        .collect();
    for filename in args.iter().skip(2) {
        let wrappers = read_lines(filename).map(|line| google::parse_semantic(&line.unwrap()));
        for wrapper in wrappers {
            if let Some(activity_segment) = wrapper.activity_segment {
                let mapped = es::map_activity_segement(activity_segment, &raw_locations);
                es::print_activity_segement(mapped);
            }
            if let Some(place_visit) = wrapper.place_visit {
                // place_visits.push(es::map_place_visit(place_visit));
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

// fn convert_activity_segment(original: ActivitySegment) -> (BulkInfo, ActivitySegment) {
//     let duration = original.duration.es_convert();
//     let start_location = original.start_location.es_convert();
//     let end_location = original.end_location.es_convert();
//     let waypoints = Some(original.es_convert());

//     let updated = ActivitySegment {
//         duration,
//         start_location,
//         end_location,
//         waypoints,
//         ..original
//     };

//     let index_info = IndexInfo {
//         index: "activitysegment".into(),
//         id: format!("{}", updated.duration.start_timestamp_ms_string),
//     };

//     let bulk_info = BulkInfo { index: index_info };

//     (bulk_info, updated)
// }

// trait EsConvertable<T> {
//     fn es_convert(&self) -> T;
// }

// impl EsConvertable<Location> for Location {
//     fn es_convert(&self) -> Location {
//         let es_location = Some(EsLocation {
//             lat: self.latitude_e7 as f64 / 1e7,
//             lon: self.longitude_e7 as f64 / 1e7,
//         });

//         Location {
//             es_location,
//             ..(*self).clone()
//         }
//     }
// }

// impl EsConvertable<EsLineString> for ActivitySegment {
//     fn es_convert(&self) -> EsLineString {
//         let coordinates = self
//             .waypoint_path
//             .waypoints
//             .iter()
//             .map(|waypoint| vec![waypoint.lng_e7 as f64 / 1e7, waypoint.lat_e7 as f64 / 1e7])
//             .collect();

//         EsLineString {
//             es_type: "linestring".into(),
//             coordinates,
//         }
//     }
// }

// impl EsConvertable<Duration> for Duration {
//     fn es_convert(&self) -> Duration {
//         let start_timestamp_ms_u64 = Some(self.start_timestamp_ms_string.parse::<u64>().unwrap());
//         let end_timestamp_ms_u64 = Some(self.end_timestamp_ms_string.parse::<u64>().unwrap());

//         Duration {
//             start_timestamp_ms_u64,
//             end_timestamp_ms_u64,
//             ..(*self).clone()
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_activity_segment() {
//         let json_string = r#"{"activitySegment":{"startLocation":{"latitudeE7":498746569,"longitudeE7":86486560,"placeId":"ChIJvdwoQGJwvUcRGF9pJrg4Px8","address":"S4|02\nGrafenstraße 2\n64283 Darmstadt\nDeutschland","name":"S4|02","semanticType":"TYPE_WORK","locationConfidence":41.26615},"endLocation":{"latitudeE7":498743210,"longitudeE7":86577130,"placeId":"ChIJFer2NWRwvUcRChISFK9g3cI","address":"Schloßgraben 1\n64283 Darmstadt\nDeutschland","name":"Darmstadtium - Science and Congress Center","locationConfidence":38.52593},"duration":{"startTimestampMs":"1354530532487","endTimestampMs":"1354530712585"},"distance":372,"confidence":"MEDIUM","activities":[{"activityType":"WALKING","probability":0},{"activityType":"CYCLING","probability":0},{"activityType":"IN_VEHICLE","probability":0},{"activityType":"FLYING","probability":0}],"waypointPath":{"waypoints":[{"latE7":498776969,"lngE7":86550121},{"latE7":498756523,"lngE7":86569156}]},"editConfirmationStatus":"NOT_CONFIRMED"}}"#;
//         let record: RecordWrapper = parse_line(json_string.into()).unwrap();

//         assert!(record.activity_segment.is_some());
//         assert!(record.place_visit.is_none());

//         let activity_segment = record.activity_segment.unwrap();
//         assert_eq!(activity_segment.activities.len(), 4);
//         assert_eq!(activity_segment.waypoint_path.waypoints.len(), 2);
//     }
// }

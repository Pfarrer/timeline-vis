use crate::model::Location;
use crate::utils::month_to_short_str;
use chrono::{Datelike, Local};
use elastic::SyncClient;
use elastic::client::requests::bulk::bulk;
use crate::elastic::prelude::DocumentType;
use elastic::endpoints::IndicesExistsRequest;
use elastic::http::StatusCode;
use elastic::prelude::StaticIndex;

pub fn index<It: Iterator<Item=Result<Location, String>>>(location_iterator: It) {
    let mut client = SyncClient::builder().build()
        .expect("Failed to connect to Elasticsearch");

    ensure_index_exists(&mut client);

    let mut last_year_month = (0, 0);
    for location_res in location_iterator {
        if location_res.is_ok() {
            let location = location_res.unwrap();
            let this_year_month = (location.timestamp.year(), location.timestamp.month());
            if last_year_month != this_year_month {
                last_year_month = this_year_month;
                println!("Indexing {} {}", month_to_short_str(this_year_month.1 as u8), this_year_month.0);
            }

            client.document().index(location).send()
                .expect("Failed to index document");
//            index_location(&mut client, location);
        } else {
            println!("Location skipped: {}", location_res.unwrap_err());
        }
    }
}

//fn index_location(client: &mut SyncClient, location: Location) {
//    let ops = vec![ bulk::<Location>().index(location)];
//
//    client.bulk()
//        .index("timeline")
//        .ty("location")
//        .extend(ops)
//        .send().unwrap();
//
//    let doc = Location::index_mapping();
//
//    let mapping = serde_json::to_string(&doc).unwrap();
//    let esdoc = serde_json::to_string(&location).unwrap();
//
//    panic!(esdoc);
//}

fn ensure_index_exists(client: &mut SyncClient) {
    if !client
        .index(Location::static_index())
        .exists()
        .send()
        .expect("Failed to fetch index from Elasticsearch")
        .exists()
    {
        client.index(Location::static_index()).create().send()
            .expect("Failed to create Elasticsearch index");
        client.document::<Location>().put_mapping().send()
            .expect("Failed to set Elasticsearch mapping");
        println!("Index created");
    } else {
        println!("Index exists already");
    }
//    let exists = client.request(IndicesExistsRequest::for_index("timeline"))
//        .send()
//        .expect("Failed to fetch index from Elasticsearch");
//
//    match exists.status() {
//        StatusCode::OK => (),
//        StatusCode::NOT_FOUND => {
//            let index_doc = Location::index_mapping();
//            let index_doc_string = serde_json::to_string(&index_doc).unwrap();
//
//            client.index("timeline")
//                .create()
//                .body(index_doc_string)
//                .send()
//                .expect("Failed to create Elasticsearch index");
//        }
//        a => panic!("Unexpected status: {:#?}", a)
//    }
}

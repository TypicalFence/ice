use std::error::Error;
use reqwest::Client;
use crate::status::Status;
use crate::trip::TripResponse;

const URL: &'static str = "https://iceportal.de";
const STATUS_ENDPOINT : &'static str = "api1/rs/status";
const TRIP_ENDPOINT:  &'static str = "api1/rs/tripInfo/trip";
const USER_AGENT: &'static str = "librice/dev";

fn get_client() -> Client {
    Client::builder().user_agent(USER_AGENT).build().unwrap_or(Client::new())
}

pub async fn get_status() ->Result<Status, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", URL, STATUS_ENDPOINT);
    // TODO log
    //println!("{}", url);

    //let resp = client.get(url).header("Accept", "*/*").header("user-agent", "hehe").send().await?;
    let resp = get_client().get(url).send().await?;

    let text = &resp.text().await?;
    // TODO log
    // println!("{}", text);

    let nya = serde_json::from_str(&text.as_str());
    return nya.map_err(|error| Box::new(error) as Box<dyn Error>)
}

pub async fn get_trip() -> Result<TripResponse, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", URL, TRIP_ENDPOINT);
    // TODO log
    //println!("{}", url);

    //let resp = client.get(url).header("Accept", "*/*").header("user-agent", "hehe").send().await?;
    let resp = get_client().get(url).send().await?;

    let text = &resp.text().await?;
    // TODO log
     //println!("{}", text);

    let nya = serde_json::from_str(&text.as_str());
    return nya.map_err(|error| Box::new(error) as Box<dyn Error>)
}


mod status {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Status {
        pub connection: bool,
        pub service_level: String,
        pub gps_status: String,
        pub internet: String,
        pub latitude: f64,
        pub longitude: f64,
        pub tile_y: i64,
        pub tile_x: i64,
        pub series: String,
        pub server_time: i64,
        pub speed: f64,
        pub train_type: String,
        pub tzn: String,
        pub wagon_class: String,
        pub connectivity: Connectivity,
        pub bap_installed: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Connectivity {
        pub current_state: String,
        pub next_state: String,
        pub remaining_time_seconds: i64,
    }
}

mod trip {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TripResponse {
        pub trip: Trip,
        pub connection: Connection,
        pub active: Value,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Trip {
        pub trip_date: String,
        pub train_type: String,
        pub vzn: String,
        pub actual_position: i64,
        pub distance_from_last_stop: i64,
        pub total_distance: i64,
        pub stop_info: StopInfo,
        pub stops: Vec<Stop>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StopInfo {
        pub scheduled_next: String,
        pub actual_next: String,
        pub actual_last: String,
        pub actual_last_started: String,
        pub final_station_name: String,
        pub final_station_eva_nr: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Stop {
        pub station: Station,
        pub timetable: Timetable,
        pub track: Track,
        pub info: Info,
        #[serde(default)]
        pub delay_reasons: Option<Vec<DelayReason>>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Station {
        pub eva_nr: String,
        pub name: String,
        pub code: Value,
        pub geocoordinates: Geocoordinates,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Geocoordinates {
        pub latitude: f64,
        pub longitude: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Timetable {
        pub scheduled_arrival_time: Option<i64>,
        pub actual_arrival_time: Option<i64>,
        pub show_actual_arrival_time: Option<bool>,
        pub arrival_delay: String,
        pub scheduled_departure_time: Option<i64>,
        pub actual_departure_time: Option<i64>,
        pub show_actual_departure_time: Option<bool>,
        pub departure_delay: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Track {
        pub scheduled: String,
        pub actual: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Info {
        pub status: i64,
        pub passed: bool,
        pub position_status: Option<String>,
        pub distance: i64,
        pub distance_from_start: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DelayReason {
        pub code: String,
        pub text: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Connection {
        pub train_type: Value,
        pub vzn: Value,
        pub train_number: Value,
        pub station: Value,
        pub timetable: Value,
        pub track: Value,
        pub info: Value,
        pub stops: Value,
        pub conflict: String,
    }
}
mod types;
use types::Sessions;

use crate::types::AttendanceEvents;
use crate::types::Attendances;
use crate::types::Data;
use crate::types::Downloads;
use crate::types::Events;
use crate::types::JsonData;
use crate::types::Locations;
use crate::types::SessionTimes;
use crate::types::SyncQueue;
use crate::types::Syncs;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("raw.json").expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let parsed_data: JsonData = serde_json::from_str(&contents).expect("Failed to parse JSON");

    let data: Data = parsed_data.data;
    let attendance_events: Vec<AttendanceEvents> = data.attendance_events;
    let attendances: Vec<Attendances> = data.attendances;
    let downloads: Vec<Downloads> = data.downloads;
    let events: Vec<Events> = data.events;
    let locations: Vec<Locations> = data.locations;
    let session_times: Vec<SessionTimes> = data.session_times;
    let sessions: Vec<Sessions> = data.sessions;
    let sync_queue: Vec<SyncQueue> = data.sync_queue;
    let syncs: Vec<Syncs> = data.syncs;

    attendance_events
        .iter()
        .for_each(|attendance_event| println!("{:?}", attendance_event.created_at));

    attendances
        .iter()
        .for_each(|attendance| println!("{:?}", attendance.name));
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AttendanceEvents {
    pub id: String,
    #[serde(rename = "attendeeId")]
    pub attendee_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub values: Values,
    pub reason: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}

#[derive(Debug, Deserialize)]
pub struct Values {
    pub changes: Option<String>,
    #[serde(rename = "contactId")]
    pub contact_id: Option<i32>,
    #[serde(rename = "entityId")]
    pub entity_id: Option<i32>,
    #[serde(rename = "eventId")]
    pub event_id: Option<i32>,
    #[serde(rename = "sourceAttendanceEventId")]
    pub source_attendance_event_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Attendances {
    pub id: i32,
    #[serde(rename = "entityId")]
    pub entity_id: i32,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    pub status: String,
    #[serde(rename = "guestCount")]
    pub guest_count: i32,
    #[serde(rename = "contactId")]
    pub contact_id: i32,
    #[serde(rename = "contactUniqueId")]
    pub contact_unique_id: String,
    pub email: String,
    pub name: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub id: String,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub body: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}

#[derive(Debug, Deserialize)]
pub struct Events {
    pub id: i32,
    pub name: String,
    #[serde(rename = "startDate")]
    pub start_date: i32,
    #[serde(rename = "endDate")]
    pub end_date: i32,
    pub timezone: String,
    #[serde(rename = "attendanceCount")]
    pub attendance_count: i32,
    #[serde(rename = "locationId")]
    pub location_id: Option<String>,
    #[serde(rename = "refreshedAt")]
    pub refreshed_at: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Locations {
    pub id: String,
    pub title: String,
    pub address: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Deserialize)]
pub struct SessionTimes {
    pub id: i32,
    pub name: String,
    #[serde(rename = "startDate")]
    pub start_date: i32,
    #[serde(rename = "endDate")]
    pub end_date: i32,
    pub timezone: String,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "sessionId")]
    pub session_id: i32,
    #[serde(rename = "attendanceCount")]
    pub attendance_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct Sessions {
    pub id: i32,
    pub name: String,
    #[serde(rename = "startDate")]
    pub start_date: i32,
    #[serde(rename = "endDate")]
    pub end_date: i32,
    pub timezone: String,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "sessionTimeCount")]
    pub session_time_count: i32,
    #[serde(rename = "attendanceCount")]
    pub attendance_count: i32,
    #[serde(rename = "locationId")]
    pub location_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SyncQueue {
    #[serde(rename = "attendeeId")]
    pub attendee_id: i32,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "entityId")]
    pub entity_id: i32,
    #[serde(rename = "contactId")]
    pub contact_id: i32,
    pub status: String,
    pub metadata: Metadata,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub reason: String,
    #[serde(rename = "sourceAttendanceEventId")]
    pub source_attendance_event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Syncs {
    pub id: String,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub metadata: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}

#[derive(Debug, Deserialize)]
pub struct JsonData {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    #[serde(rename = "AttendanceEvents")]
    pub attendance_events: Vec<AttendanceEvents>,
    #[serde(rename = "Attendances")]
    pub attendances: Vec<Attendances>,
    #[serde(rename = "Downloads")]
    pub downloads: Vec<Downloads>,
    #[serde(rename = "Events")]
    pub events: Vec<Events>,
    #[serde(rename = "Locations")]
    pub locations: Vec<Locations>,
    #[serde(rename = "SessionTimes")]
    pub session_times: Vec<SessionTimes>,
    #[serde(rename = "Sessions")]
    pub sessions: Vec<Sessions>,
    #[serde(rename = "SyncQueue")]
    pub sync_queue: Vec<SyncQueue>,
    #[serde(rename = "Syncs")]
    pub syncs: Vec<Syncs>,
}

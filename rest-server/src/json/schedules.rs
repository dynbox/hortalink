use garde::Validate;
use serde::{Deserialize, Serialize};

use common::entities::WeekDay;

#[derive(Serialize, Deserialize, Validate)]
#[garde(allow_unvalidated)]
pub struct CreateSchedulePayload {
    pub location: ScheduleLocation,
    #[garde(length(min = 5, max = 256))]
    pub address: String,
    pub start_time: time::Time,
    pub end_time: time::Time,
    pub day_of_week: WeekDay,
}

#[derive(Serialize, Deserialize)]
pub struct ScheduleLocation {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Validate)]
#[garde(allow_unvalidated)]
pub struct UpdateSchedulePayload {
    pub location: Option<ScheduleLocation>,
    #[garde(length(min = 5, max = 256))]
    pub address: Option<String>,
    pub start_time: Option<time::Time>,
    pub end_time: Option<time::Time>,
    pub day_of_week: Option<WeekDay>,
}
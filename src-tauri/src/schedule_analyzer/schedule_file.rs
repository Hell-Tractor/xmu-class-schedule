use serde::{Serialize, Deserialize};

use super::ClassSchedule;

#[derive(Serialize, Deserialize)]
pub struct ScheduleFile {
    pub times: Vec<(String, String)>,
    pub schedules: Vec<ClassSchedule>
}

impl ScheduleFile {
    pub fn new() -> ScheduleFile {
        ScheduleFile {
            times: Vec::new(),
            schedules: Vec::new()
        }
    }
}
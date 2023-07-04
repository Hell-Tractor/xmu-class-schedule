use super::class_week::ClassWeek;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ClassSchedule {
    pub name: String,
    pub teacher: String,
    pub start_time: String,
    pub week_day: u8,
    pub span: u8,
    pub location: String,
    pub weeks: ClassWeek,
}

impl PartialEq for ClassSchedule {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.teacher == other.teacher
    }
}
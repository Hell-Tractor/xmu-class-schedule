use std::collections::HashMap;

use serde::{Serialize, ser::SerializeSeq};
use tauri::InvokeError;

use crate::schedule_analyzer::{ScheduleParseError, self, ScheduleFile};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}.")]
    IoError(#[from] std::io::Error),
    #[error("Error when parsing schedule: {0}.")]
    DeserializeError(#[from] serde_json::Error),
}

impl Into<InvokeError> for Error {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}

type Result<T> = std::result::Result<T, Error>;

fn load_schedule(name: &str) -> Result<ScheduleFile> {
    let file_path = format!("../data/schedules/{name}.json");
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let schedules: ScheduleFile = serde_json::from_reader(reader)?;
    Ok(schedules)
}

#[tauri::command]
pub fn parse_schedule(input_file_path: &str, name: &str) -> std::result::Result<(), ScheduleParseError> {
    schedule_analyzer::parse(input_file_path, format!("../data/schedules/{name}.json").as_str())
}

pub enum TimeOrSchedule {
    Time(String, String),
    Schedule(schedule_analyzer::ClassSchedule),
}

impl Serialize for TimeOrSchedule {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            TimeOrSchedule::Time(start, end) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(start)?;
                seq.serialize_element(end)?;
                seq.end()
            },
            TimeOrSchedule::Schedule(schedule) => schedule.serialize(serializer),
        }
    }
}

pub fn find_schedule(schedules: &Vec<schedule_analyzer::ClassSchedule>, week: u8, day: u8, time: String) -> Option<schedule_analyzer::ClassSchedule> {
    schedules.iter()
        .find(|schedule| schedule.weeks.contains(week) && schedule.week_day == day && schedule.start_time == time)
        .map(|schedule| schedule.clone())
}

#[tauri::command]
pub fn get_schedules(name: &str, week: u8) -> Result<Vec<HashMap<String, TimeOrSchedule>>> {
    println!("get_schedules: name: {}, week: {}", name, week);
    // let result: Vec<schedule_analyzer::ClassSchedule> = load_schedule(name)?.into_iter()
    //     .filter(|class| class.weeks.contains(week))
    //     .collect();
    let mut result: Vec<HashMap<String, TimeOrSchedule>> = Vec::new();
    let schedule = load_schedule(name)?;
    for i in 0..schedule.times.len() {
        let mut map = HashMap::new();
        map.insert("time".to_string(), TimeOrSchedule::Time(schedule.times[i].0.clone(), schedule.times[i].1.clone()));
        for day in 1..=7 {
            let s = find_schedule(&schedule.schedules, week, day, schedule.times[i].0.clone());
            if s.is_none() {
                continue;
            }
            map.insert(day.to_string(), TimeOrSchedule::Schedule(s.unwrap()));
        }
        result.push(map);
    }

    // write result to ../temp/debug.json
    // let file_path = format!("../temp/debug.json");
    // let file = std::fs::File::create(file_path)?;
    // let writer = std::io::BufWriter::new(file);
    // serde_json::to_writer(writer, &result)?;

    Ok(result)
}

#[tauri::command]
pub fn get_schedule_list() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let Ok(paths) = std::fs::read_dir("../data/schedules") else {
        return result;
    };
    for path in paths {
        let path = path.unwrap().path();
        let Some(file_name) = path.file_name() else {
            continue;
        };
        // remove extension
        let Some((name, extension)) = file_name.to_str().unwrap().split_once(".") else {
            continue;
        };
        if extension != "json" {
            continue;
        }
        result.push(name.to_string());
    }
    result
}
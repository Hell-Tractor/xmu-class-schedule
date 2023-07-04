use tauri::regex::Regex;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ClassWeekType {
    Odd,
    Even,
    All,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ClassWeek {
    start_week: u8,
    end_week: u8,
    week_type: ClassWeekType,
}

#[derive(Error, Debug)]
pub enum WeekParseError {
    #[error("Bad format.")]
    BadFormat
}

impl ClassWeek {
    pub fn new(start_week: u8, end_week: u8, week_type: ClassWeekType) -> ClassWeek {
        ClassWeek {
            start_week,
            end_week,
            week_type,
        }
    }
    pub fn from_str(value: &str) -> Result<ClassWeek, WeekParseError> {
        let re = Regex::new(r#"(?P<begin>\d+)-(?P<end>\d+)(?P<type>[单双]?)周"#).unwrap();
        let caps = re.captures(value).ok_or(WeekParseError::BadFormat)?;
        Ok(ClassWeek::new(
            caps.name("begin").unwrap().as_str().parse().unwrap(),
            caps.name("end").unwrap().as_str().parse().unwrap(),
            caps.name("type").map_or(ClassWeekType::All, |v| match v.as_str() {
                "单" => ClassWeekType::Odd,
                "双" => ClassWeekType::Even,
                _ => ClassWeekType::All,
            })
        ))
    }
    pub fn contains(&self, week: u8) -> bool {
        week >= self.start_week && week <= self.end_week && match self.week_type {
            ClassWeekType::All => true,
            ClassWeekType::Odd => week % 2 == 1,
            ClassWeekType::Even => week % 2 == 0,
        }
    }
}

#[cfg(test)]
mod parse_from_str_tests {
    use super::ClassWeek;
    use super::ClassWeekType;

    #[test]
    fn test1() {
        assert_eq!(ClassWeek::from_str("1-16周").unwrap(), ClassWeek::new(1, 16, ClassWeekType::All))
    }

    #[test]
    fn test2() {
        assert_eq!(ClassWeek::from_str("1-16单周").unwrap(), ClassWeek::new(1, 16, ClassWeekType::Odd))
    }

    #[test]
    fn test3() {
        assert_eq!(ClassWeek::from_str("1-16双周").unwrap(), ClassWeek::new(1, 16, ClassWeekType::Even))
    }

    #[test]
    #[should_panic]
    fn test4() {
        ClassWeek::from_str("1-16").unwrap();
    }
}
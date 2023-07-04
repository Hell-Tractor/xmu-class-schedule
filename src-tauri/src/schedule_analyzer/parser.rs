use std::{fs::File, io::Read};

use scraper::ElementRef;
use thiserror::Error;

use super::{class_schedule::ClassSchedule, class_week::ClassWeek, schedule_file::ScheduleFile};

#[derive(Error, Debug)]
pub enum ScheduleParseError {
    #[error("IO Error: {0}.")]
    IoError(#[from] std::io::Error),
    #[error("Invalid html format: {0}.")]
    FormatError(String),
    #[error(transparent)]
    WeekParseError(#[from] super::class_week::WeekParseError),
    #[error("Error when serializing: {0}.")]
    SerializeError(#[from] serde_json::Error),
    #[error("Error when parsing week day: {0}.")]
    WeekDayParseError(#[from] std::num::ParseIntError)
}

pub fn parse(html: &str, output_file_path: &str) -> Result<(), ScheduleParseError> {
    // read and parse html
    let mut doc = String::new();
    File::open(html)?.read_to_string(&mut doc)?;
    let doc = scraper::Html::parse_document(&doc);

    // select class schedule table
    let table_selector = scraper::Selector::parse(r#"table[id="jsTbl_01"]"#).unwrap();
    let table = doc.select(&table_selector).next().ok_or(ScheduleParseError::FormatError("Table not found".to_string()))?;

    // select rows
    let row_selector = scraper::Selector::parse("tr").unwrap();
    let rows = table.select(&row_selector).skip(1);

    // parse classes and times
    let mut result = ScheduleFile::new();
    let time_td_selector = scraper::Selector::parse(r#"td[style="font-weight:normal;"]"#).unwrap();
    for row in rows {
        let begin_time = parse_begin_time(&row)?;
        result.schedules.append(&mut parse_class(&row, &begin_time)?);

        let time_td = row.select(&time_td_selector).next().ok_or(ScheduleParseError::FormatError("Time td not found".to_string()))?;
        let time = time_td.inner_html().split('~').map(|t| t.trim().to_string()).collect::<Vec<_>>();
        if time.len() != 2 {
            return Err(ScheduleParseError::FormatError("Invalid time format".to_string()));
        }
        result.times.push((time[0].to_string(), time[1].to_string()));
    }

    // write to file
    let output_file = File::create(output_file_path)?;
    serde_json::to_writer(output_file, &result)?;

    Ok(())
}

fn parse_class(row: &ElementRef, begin_time: &String) -> Result<Vec<ClassSchedule>, ScheduleParseError> {
    let selector = scraper::Selector::parse(r#"div[class="arrage"]"#).unwrap();
    let mut result = Vec::<ClassSchedule>::new();
    for class_div in row.select(&selector) {
        let td = class_div.parent()
            .ok_or(ScheduleParseError::FormatError("Class div has no parent".to_string()))?
            .value()
            .as_element()
            .ok_or(ScheduleParseError::FormatError("Class div parent is not element".to_string()))?;
        let span = td.attr("rowspan")
            .unwrap_or("1")
            .parse::<u8>()
            .unwrap();
        let week_day = td.attr("xq")
            .ok_or(ScheduleParseError::FormatError("Class div parent has no xq attribute".to_string()))?
            .parse::<u8>()?;
        let style = td.attr("style")
            .unwrap_or("")
            .contains("display: none;");
        if style {
            continue;
        }
        let text = class_div.text().map(|s| s.trim()).filter(|s| s.len() > 0).collect::<Vec<_>>();
        if text.len() != 5 {
            return Err(ScheduleParseError::FormatError(format!("Class div children count is not 5, {:?}", text)));
        }
        let class = ClassSchedule {
            name: text[2].to_string(),
            teacher: text[3].to_string(),
            start_time: begin_time.clone(),
            week_day,
            span,
            location: text[4].to_string(),
            weeks: ClassWeek::from_str(text[1])?
        };
        result.push(class);
    }
    Ok(result)
}

fn parse_begin_time(row: &ElementRef) -> Result<String, ScheduleParseError> {
    let selector = scraper::Selector::parse(r#"[style="font-weight:normal;"]"#).unwrap();
    let time = row.select(&selector)
        .next()
        .ok_or(ScheduleParseError::FormatError("Begin time not found".to_string()))?
        .inner_html()
        .split("~")
        .next()
        .ok_or(ScheduleParseError::FormatError("Begin time format invalid".to_string()))?
        .trim()
        .to_string();
    Ok(time)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        super::parse("../temp/我的课表app.html", "../temp/我的课表app.json").unwrap();
    }
}
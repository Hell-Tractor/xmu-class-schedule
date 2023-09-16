use serde::{Serialize, Deserialize};
use tauri::InvokeError;

const CONFIG_FILE_PATH: &str = "../config.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub start_of_semester: String,
    pub default_schedule_name: String
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}.")]
    IoError(#[from] std::io::Error),
    #[error("Error when parsing settings: {0}.")]
    DeserializeError(#[from] serde_json::Error)
}

type Result<T> = std::result::Result<T, Error>;

impl Into<InvokeError> for Error {
    fn into(self) -> InvokeError {
        InvokeError::from(self.to_string())
    }
}

impl Settings {
    pub fn default() -> Settings {
        Settings {
            start_of_semester: String::from("2023-09-10"),
            default_schedule_name: String::from("无课程表")
        }
    }
}

#[tauri::command]
pub fn get_or_default_settings() -> Result<Settings> {
    let Ok(file) = std::fs::File::open(CONFIG_FILE_PATH) else {
        println!("get_or_default_settings: config file not found, using default settings.");
        return Ok(Settings::default());
    };
    let reader = std::io::BufReader::new(file);
    let settings: Settings = serde_json::from_reader(reader)?;
    println!("get_or_default_settings: {:?}", settings);
    Ok(settings)
}

#[tauri::command]
pub fn set_settings(settings: Settings) -> Result<()> {
    println!("set_settings: {:?}", settings);
    let file = std::fs::File::create(CONFIG_FILE_PATH)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, &settings)?;
    Ok(())
}
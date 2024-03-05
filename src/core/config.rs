use crate::core::prayertime::PrayerTime;

pub struct Config {
    pub prayers: Vec<PrayerTime>
}

impl Config {
    fn get_config_path() -> String { "".to_string() }
}
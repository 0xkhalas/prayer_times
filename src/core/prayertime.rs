#[derive(Debug)]
pub struct PrayerTime {
    pub name: String,
    pub is_pm: bool,
    pub hour: u32,
    pub minute: u32,
}

impl PrayerTime {
    pub fn new(name: &str, is_pm: bool, hour: u32, minute: u32) -> PrayerTime {
        PrayerTime {
            name: name.to_string(),
            is_pm,
            hour,
            minute
        }
    }

    pub fn check_time(&self, is_pm: bool, hour: u32, minute: u32, less_time: i32) -> bool {
        self.is_pm == is_pm &&
            self.hour == hour &&
            (((minute as i32) - (self.minute as i32)) >= 0 && ((minute as i32) - (self.minute as i32)) <= less_time)
    }
}


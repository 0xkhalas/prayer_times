use std::cmp::min;
use std::fmt::format;
use chrono::Timelike;
use crate::core::config::Config;
use crate::core::prayertime::PrayerTime;
use crate::core::sound::play_audio;

pub struct Prayer {
    config: Config,
    is_called: bool
}

impl Prayer {
    pub fn new(config: Config) -> Prayer {
        Prayer {
            config,
            is_called: false
        }
    }

    fn is_prayer_time(&self, time: &PrayerTime) -> bool {
        let current = chrono::Local::now();
        let (is_pm, hour)  = current.hour12();
        let minute = current.minute();

        time.check_time(is_pm, hour, minute, 10)
    }

    fn get_prayer(&self) -> Option<&PrayerTime> {
        for prayer in &self.config.prayers {
            if self.is_prayer_time(&prayer) {
                return Some(prayer);
            }
        }
        None
    }

    pub fn handle_time(&mut self) {
        if let Some(i) = self.get_prayer() {
            if self.is_called == false {
                play_audio("assets/sound.mp3");
            }
            self.is_called = true
        } else {
            self.is_called = false;
        }
    }
}

#[test]
fn asd() {



}
use std::time::Duration;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_sleep: u32,
        counter_tick: u64,
    },
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        Self::Initialized {
            duration,
            counter_sleep,
            counter_tick,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn increment_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let updated_duration = duration.as_secs() + 1;
            if updated_duration > 10 {
                log::error!("Cannot increment more than 10 seconds");
            }

            let clamped_duration = updated_duration.clamp(1, 10);
            *duration = Duration::from_secs(clamped_duration);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let updated_duration = duration.as_secs() - 1;
            if updated_duration < 1 {
                log::error!("Cannot decrement less than 1 second");
            }
                        
            let clamped_duration = updated_duration.clamp(1, 10);
            *duration = Duration::from_secs(clamped_duration);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
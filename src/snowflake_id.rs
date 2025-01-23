const TWITTER_EPOCH: i64 = 1288834974657;

use std::time::{SystemTime, UNIX_EPOCH};

pub struct SnowflakeIdGenerator {
    worker_id: u16,
    last_timestamp: i64,
    sequence: u16,
}

impl SnowflakeIdGenerator {
    pub fn new() -> Self {
        SnowflakeIdGenerator {
            worker_id: 0,
            last_timestamp: 0,
            sequence: 0,
        }
    }

    pub fn generate(&mut self) -> i64 {
        let mut timestamp = Self::current_time_millis();
        
        if timestamp < self.last_timestamp {
            panic!("Clock moved backwards. Refusing to generate id");
        }

        if self.last_timestamp == timestamp {
            self.sequence = (self.sequence + 1) & 0xFFF;
            if self.sequence == 0 {
                timestamp = Self::wait_for_next_millis(self.last_timestamp);
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        ((timestamp - TWITTER_EPOCH) << 22)
            | ((self.worker_id as i64) << 12)
            | (self.sequence as i64)
    }

    fn current_time_millis() -> i64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_millis() as i64
    }

    fn wait_for_next_millis(last_timestamp: i64) -> i64 {
        let mut timestamp = Self::current_time_millis();
        while timestamp <= last_timestamp {
            timestamp = Self::current_time_millis();
        }
        timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_unique_ids() {
        let mut generator = SnowflakeIdGenerator::new();
        let id1 = generator.generate();
        let id2 = generator.generate();
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }

    #[test]
    fn test_generate_id_within_same_millisecond() {
        let mut generator = SnowflakeIdGenerator::new();
        generator.last_timestamp = SnowflakeIdGenerator::current_time_millis();
        let id1 = generator.generate();
        let id2 = generator.generate();
        assert_ne!(id1, id2, "Generated IDs within the same millisecond should be unique");
    }

    #[test]
    #[should_panic(expected = "Clock moved backwards. Refusing to generate id")]
    fn test_clock_moved_backwards() {
        let mut generator = SnowflakeIdGenerator::new();
        generator.last_timestamp = SnowflakeIdGenerator::current_time_millis();
        generator.last_timestamp += 1;
        generator.generate();
    }
}

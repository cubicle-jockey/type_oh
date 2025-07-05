use crate::ascii_chars::AsciiChars;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum HitType {
    /// Hit with a timestamp and the reaction time in milliseconds
    Hit(NaiveDateTime, u64),
    /// Miss with a timestamp
    Miss(NaiveDateTime),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatRecord {
    pub char_id: AsciiChars,
    pub results: Vec<HitType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    records: HashMap<AsciiChars, StatRecord>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            records: HashMap::new(),
        }
    }

    pub fn add_hit(&mut self, char_id: AsciiChars, timestamp: NaiveDateTime, reaction_time: u64) {
        let record = self.records.entry(char_id.clone()).or_insert(StatRecord {
            char_id,
            results: Vec::new(),
        });
        record.results.push(HitType::Hit(timestamp, reaction_time));
    }

    pub fn add_miss(&mut self, char_id: AsciiChars, timestamp: NaiveDateTime) {
        let record = self.records.entry(char_id.clone()).or_insert(StatRecord {
            char_id,
            results: Vec::new(),
        });
        record.results.push(HitType::Miss(timestamp));
    }

    pub fn get_records(&self) -> &HashMap<AsciiChars, StatRecord> {
        &self.records
    }

    pub fn get_record(&self, char_id: &AsciiChars) -> Option<&StatRecord> {
        self.records.get(char_id)
    }
}

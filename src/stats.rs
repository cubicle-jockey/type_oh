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

#[derive(Debug)]
pub struct StatSummary {
    pub total_hits: usize,
    pub total_misses: usize,
    pub hit_low: u64,
    pub hit_high: u64,
    pub hit_avg: u64,
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

    pub fn get_total_hit_count(&self) -> usize {
        self.records
            .values()
            .map(|r| {
                r.results
                    .iter()
                    .filter(|&hit| matches!(hit, HitType::Hit(_, _)))
                    .count()
            })
            .sum()
    }

    pub fn get_total_miss_count(&self) -> usize {
        self.records
            .values()
            .map(|r| {
                r.results
                    .iter()
                    .filter(|&hit| matches!(hit, HitType::Miss(_)))
                    .count()
            })
            .sum()
    }

    pub fn get_char_stats(&self, char_id: &AsciiChars) -> Option<StatSummary> {
        self.records.get(char_id).map(|record| {
            let hits: Vec<u64> = record
                .results
                .iter()
                .filter_map(|hit| {
                    if let HitType::Hit(_, reaction_time) = hit {
                        Some(*reaction_time)
                    } else {
                        None
                    }
                })
                .collect();

            let total_hits = hits.len();
            let total_misses = record.results.len() - total_hits;
            let hit_low = *hits.iter().min().unwrap_or(&0);
            let hit_high = *hits.iter().max().unwrap_or(&0);
            let hit_avg = if total_hits > 0 {
                hits.iter().sum::<u64>() as f64 / total_hits as f64
            } else {
                0.0
            }
            .floor() as u64;

            StatSummary {
                total_hits,
                total_misses,
                hit_low,
                hit_high,
                hit_avg,
            }
        })
    }

    pub fn generate_html_report(&self) -> String {
        let mut html = String::new();
        html.push_str("<html><body><h1>Stats Report</h1><table border='1'>");
        html.push_str("<tr><th>Character</th><th>Hits</th><th>LowMS</th><th>HighMS</th><th>AvgMS</th><th>Misses</th></tr>");

        let mut keys: Vec<&AsciiChars> = self.records.keys().collect();
        keys.sort_unstable();

        for (char_id) in keys {
            let stats = self.get_char_stats(char_id).unwrap_or(StatSummary {
                total_hits: 0,
                total_misses: 0,
                hit_low: 0,
                hit_high: 0,
                hit_avg: 0,
            });

            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                char_id.as_char(),
                stats.total_hits,
                stats.hit_low,
                stats.hit_high,
                stats.hit_avg,
                stats.total_misses
            ));
        }

        html.push_str("</table></body></html>");
        html
    }
}

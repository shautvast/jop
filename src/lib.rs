use core::f32;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub struct GcLine {
    pub ts: f32,
    pub used_before: usize,
    pub used_after: usize,
    pub total_memory: usize,
    pub duration: f32,
}

static GC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\[(\d+\.\d+)(.)]\[info]\[gc].+?\s(\d+)(.)->(\d+)(.)\((\d+)(.)\)\s(\d+\.\d+)(.+)"#).unwrap()
});

pub fn parse(line_str: &str) -> Option<GcLine> {
    if let Some(captures) = GC.captures(line_str) {
        let _qunit_ts = captures.get(2).unwrap().as_str();
        let ts = f32::from_str(captures.get(1).unwrap().as_str()).unwrap();
        let unit_ub = parse_mem_unit(captures.get(4).unwrap().as_str());
        let used_before = usize::from_str(captures.get(3).unwrap().as_str()).unwrap() * unit_ub;
        let unit_ua = parse_mem_unit(captures.get(6).unwrap().as_str());
        let used_after = usize::from_str(captures.get(5).unwrap().as_str()).unwrap() * unit_ua;
        let unit_tm = parse_mem_unit(captures.get(8).unwrap().as_str());
        let total_memory = usize::from_str(captures.get(7).unwrap().as_str()).unwrap() * unit_tm;
        let duration = f32::from_str(captures.get(9).unwrap().as_str()).unwrap();

        Some(GcLine { ts, used_before, used_after, total_memory, duration })
    } else {
        None
    }
}


fn parse_mem_unit(value: &str) -> usize {
    match value {
        "K" => 1_024,
        "M" => 1_048_576,
        "G" => 1_073_741_824,
        _ => 1
    }
}
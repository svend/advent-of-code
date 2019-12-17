use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Event {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Eq)]
struct Entry {
    time: (u32, u32, u32, u32, u32),
    event: Event,
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.time == other.time
    }
}

impl Entry {
    fn minute(&self) -> u32 {
        self.time.3 * 60 + self.time.4
    }
}

#[derive(Clone, Debug)]
struct Nap {
    id: u32,
    start: u32,
    end: u32,
}

impl Nap {
    fn duration(&self) -> u32 {
        self.end - self.start
    }
}

fn parse_line(s: &str) -> Entry {
    let parts: Vec<_> = s.splitn(3, ' ').collect();

    let date: Vec<_> = parts[0]
        .trim_start_matches('[')
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let time: Vec<_> = parts[1]
        .trim_end_matches(']')
        .splitn(2, ':')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let time = (date[0], date[1], date[2], time[0], time[1]);

    let message = parts[2];

    let event = match message {
        "falls asleep" => Event::FallsAsleep,
        "wakes up" => Event::WakesUp,
        s => {
            let id = s
                .split_whitespace()
                .find(|s| s.starts_with('#'))
                .unwrap()
                .trim_start_matches('#')
                .parse::<u32>()
                .unwrap();
            Event::BeginsShift(id)
        }
    };

    Entry { time, event }
}

fn max_time(naps: &[&Nap]) -> u32 {
    let mut occurences = HashMap::new();
    for nap in naps {
        for i in nap.start..nap.end {
            *occurences.entry(i).or_insert(0) += 1;
        }
    }

    occurences.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

fn max_time_2(naps: &[Nap]) -> (u32, u32) {
    let mut occurences = HashMap::new();
    for nap in naps {
        for i in nap.start..nap.end {
            *occurences.entry((nap.id, i)).or_insert(0) += 1;
        }
    }

    occurences.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

fn lines_to_naps(lines: &[&str]) -> Vec<Nap> {
    let mut naps: Vec<Nap> = vec![];

    let mut nap = Nap {
        id: 0,
        start: 0,
        end: 0,
    };

    let mut entries: Vec<_> = lines.iter().map(|l| parse_line(&l)).collect();
    entries.sort();

    for entry in &entries {
        match entry.event {
            Event::BeginsShift(id) => nap.id = id,
            Event::FallsAsleep => nap.start = entry.minute(),
            Event::WakesUp => {
                nap.end = entry.minute();
                naps.push(nap.clone())
            }
        }
    }

    naps
}

fn id_times_minute(lines: &[&str]) -> u32 {
    let naps = lines_to_naps(lines);

    let mut durations = HashMap::new();

    for nap in &naps {
        *durations.entry(nap.id).or_insert(0) += nap.duration();
    }

    let id_max = durations
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .0;
    let naps_for: Vec<_> = naps.iter().filter(|n| &n.id == id_max).collect();
    let mtime = max_time(&naps_for);

    id_max * mtime
}

fn id_times_minute_2(lines: &[&str]) -> u32 {
    let naps = lines_to_naps(lines);
    let (id, minute) = max_time_2(&naps);

    id * minute
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_times_minute() {
        let input = include_str!("4.example.input");
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(id_times_minute(&lines), 240);

        let input = include_str!("4.input");
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(id_times_minute(&lines), 35623);
    }

    #[test]
    fn test_foo() {
        let input = include_str!("4.example.input");
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(id_times_minute_2(&lines), 4455);

        let input = include_str!("4.input");
        let lines: Vec<_> = input.lines().collect();
        assert_eq!(id_times_minute_2(&lines), 23037);
    }
}

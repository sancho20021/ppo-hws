use std::{collections::HashMap, time::Instant};

use crate::clock::Clock;

pub trait EventsStatistics {
    fn inc_event(&mut self, name: &str);
    /// average rpm for the past hour
    fn get_event_statistic_by_name(&self, name: &str) -> usize;
    /// average rpms for the past hour
    fn get_all_events_statistic(&self) -> HashMap<String, usize>;
    /// print average rpms for the past hour
    fn print_statistic(&self);
}

struct Measurement {
    /// timestamp of the 0th minute from the array
    timestamp: Instant,
    /// array of rmps (from newest to oldest)
    rpms: [usize; 60],
}

impl Measurement {
    fn empty(when: Instant) -> Measurement {
        Measurement {
            timestamp: when,
            rpms: [0; 60],
        }
    }
}

pub struct EventsStaisticImpl<C>
where
    C: Clock,
{
    clock: C,
    events: HashMap<String, Measurement>,
}

impl<C> EventsStaisticImpl<C>
where
    C: Clock,
{
    pub fn new(clock: C) -> Self {
        EventsStaisticImpl { clock, events: Default::default() }
    }

    fn get_event_stat_by_name_by_time(&self, name: &str, time: Instant) -> usize {
        let empty_meas = Measurement::empty(time);
        let meas = self.events.get(name).unwrap_or(&empty_meas);
        let no_events_for = std::cmp::min(((time - meas.timestamp).as_secs() / 60) as usize, 60);
        vec![0; no_events_for]
            .iter()
            .chain(meas.rpms[no_events_for..].iter())
            .map(|x| *x)
            .sum::<usize>()
            / 60
    }
}

impl<C> EventsStatistics for EventsStaisticImpl<C>
where
    C: Clock,
{
    fn inc_event(&mut self, name: &str) {
        let now = self.clock.now();
        if !self.events.contains_key(name) {
            self.events.insert(name.to_string(), Measurement::empty(now));
        }
        let mut meas = self.events.get_mut(name).unwrap();
        let no_events_for = std::cmp::min(((now - meas.timestamp).as_secs() / 60) as usize, 60);
        meas.rpms.rotate_right(no_events_for);
        (&mut meas.rpms)[0..no_events_for].fill(0);
        meas.rpms[0] += 1;
        meas.timestamp = now;
    }

    fn get_event_statistic_by_name(&self, name: &str) -> usize {
        self.get_event_stat_by_name_by_time(name, self.clock.now())
    }

    fn get_all_events_statistic(&self) -> HashMap<String, usize> {
        let now = self.clock.now();
        self.events
            .keys()
            .map(|k| (k.clone(), self.get_event_stat_by_name_by_time(k, now)))
            .collect()
    }

    fn print_statistic(&self) {
        println!("Events for the past hour:");
        let stats = self.get_all_events_statistic();
        for (event, rpm) in stats {
            println!("{}: {} rpm", event, rpm);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Instant};

    use crate::clock::{FrozenClock, Clock};

    use super::{EventsStaisticImpl, EventsStatistics};

    fn happen<C : Clock>(es: &mut EventsStaisticImpl<C>, event: &str, n: usize) {
        for _ in 0..n {
            es.inc_event(event);
        }
    }

    #[test]
    fn test1() {
        let mut es = EventsStaisticImpl::new(FrozenClock::new(Instant::now()));
        happen(&mut es, "hello", 60);
        assert_eq!(1, es.get_event_statistic_by_name("hello"))
    }

    #[test]
    fn test2() {
        let now = Instant::now();
        let mut es = EventsStaisticImpl::new(FrozenClock::new(now));
        for _ in 0..60 {
            es.clock.skip_minutes(1);
            happen(&mut es, "hello", 2);
        }
        assert_eq!(2, es.get_event_statistic_by_name("hello"));
        es.clock.skip_minutes(30);
        assert_eq!(1, es.get_event_statistic_by_name("hello"));
        es.clock.skip_minutes(30);
        assert_eq!(0, es.get_event_statistic_by_name("hello"));
    }

    #[test]
    fn test3() {
        let now = Instant::now();
        let mut es = EventsStaisticImpl::new(FrozenClock::new(now));
        happen(&mut es, "hello", 60);
        happen(&mut es, "yes", 120);
        assert_eq!(1, es.get_event_statistic_by_name("hello"));
        assert_eq!(2, es.get_event_statistic_by_name("yes"));
    }
}

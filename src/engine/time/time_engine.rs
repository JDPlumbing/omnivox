use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_display::{format_simtime, 
                                    format_simdate, 
                                    TimeFormat
                                };
use crate::core::tdt::sim_julian::simtime_to_julian;
use crate::core::tdt::sim_duration::SimDuration;
use crate::engine::time::types::{TimeDelta, 
                                FormattedSimTime, 
                                HumanDuration, 
                                JulianDateResult, 
                                SimDateResult
                            };

#[derive(Debug, Clone, Default)]
pub struct TimeEngine;

impl TimeEngine {
    pub fn now(&self) -> SimTime {
        SimTime::now()
    }

    pub fn format_simtime(&self, ns: i128, fmt: TimeFormat) -> FormattedSimTime {
        let t = SimTime::from_ns(ns);

        FormattedSimTime {
            formatted: format_simtime(t, fmt),
            format: fmt,
            ns,
        }
    }
    pub fn delta_between(&self, start_ns: i128, end_ns: i128) -> TimeDelta {
        let start = SimTime::from_ns(start_ns);
        let end = SimTime::from_ns(end_ns);

        let delta: SimDuration = end - start;

        TimeDelta {
            delta_ns: delta.as_ns(),
            human: delta.to_string_human(),
        }
    }

    pub fn human_duration(&self, ns: i128) -> HumanDuration {
        let dur = SimDuration::from_ns(ns);

        HumanDuration {
            ns,
            human: dur.to_string_human(),
        }
    }

    pub fn julian_from_ns(&self, ns: i128) -> JulianDateResult {
        let t = SimTime::from_ns(ns);
        let jd = simtime_to_julian(t);

        JulianDateResult {
            julian_date: jd,
            ns,
        }
    }

    pub fn simdate_from_ns(&self, ns: i128) -> SimDateResult {
        let t = SimTime::from_ns(ns);
        let simdate = format_simdate(t);

        SimDateResult {
            simdate,
            ns,
        }
    }
}

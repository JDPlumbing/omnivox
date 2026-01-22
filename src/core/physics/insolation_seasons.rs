use crate::core::tdt::SimTime;

#[derive(Debug, Clone)]
pub struct InsolationSample {
    pub time: SimTime,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct InsolationSeason {
    pub time: SimTime,
    pub insolation: f64,
}

#[derive(Debug, Clone)]
pub struct InsolationSeasons {
    pub summer_solstice: Option<InsolationSeason>,
    pub winter_solstice: Option<InsolationSeason>,
    pub vernal_equinox: Option<InsolationSeason>,
    pub autumnal_equinox: Option<InsolationSeason>,
}

pub fn detect_insolation_seasons(
    samples: &[InsolationSample],
) -> InsolationSeasons {
    if samples.len() < 3 {
        return InsolationSeasons {
            summer_solstice: None,
            winter_solstice: None,
            vernal_equinox: None,
            autumnal_equinox: None,
        };
    }

    let mut summer: Option<InsolationSeason> = None;
    let mut winter: Option<InsolationSeason> = None;
    let mut vernal: Option<(SimTime, f64)> = None;     // store slope
    let mut autumnal: Option<(SimTime, f64)> = None;

    for i in 1..samples.len() - 1 {
        let prev = &samples[i - 1];
        let curr = &samples[i];
        let next = &samples[i + 1];

        let d_prev = curr.value - prev.value;
        let d_next = next.value - curr.value;
        let slope = (next.value - prev.value) * 0.5;

        // Summer solstice (maximum)
        if d_prev > 0.0 && d_next < 0.0 {
            if summer
                .as_ref()
                .map(|s| curr.value > s.insolation)
                .unwrap_or(true)
            {
                summer = Some(InsolationSeason {
                    time: curr.time,
                    insolation: curr.value,
                });
            }
        }

        // Winter solstice (minimum)
        if d_prev < 0.0 && d_next > 0.0 {
            if winter
                .as_ref()
                .map(|s| curr.value < s.insolation)
                .unwrap_or(true)
            {
                winter = Some(InsolationSeason {
                    time: curr.time,
                    insolation: curr.value,
                });
            }
        }

        // Vernal equinox (max positive slope)
        if slope > 0.0 {
            if vernal
                .as_ref()
                .map(|(_, s)| slope > *s)
                .unwrap_or(true)
            {
                vernal = Some((curr.time, slope));
            }
        }

        // Autumnal equinox (max negative slope)
        if slope < 0.0 {
            if autumnal
                .as_ref()
                .map(|(_, s)| slope < *s)
                .unwrap_or(true)
            {
                autumnal = Some((curr.time, slope));
            }
        }
    }

    InsolationSeasons {
        summer_solstice: summer,
        winter_solstice: winter,
        vernal_equinox: vernal.map(|(t, _)| InsolationSeason {
            time: t,
            insolation: samples.iter().find(|s| s.time == t).unwrap().value,
        }),
        autumnal_equinox: autumnal.map(|(t, _)| InsolationSeason {
            time: t,
            insolation: samples.iter().find(|s| s.time == t).unwrap().value,
        }),
    }
}

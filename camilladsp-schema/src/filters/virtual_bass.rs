// CamillaDSP - A flexible tool for processing audio
// Copyright (C) 2026 Henrik Enquist
//
// This file is part of CamillaDSP.
//
// CamillaDSP is free software; you can redistribute it and/or modify it
// under the terms of either:
//
// a) the GNU General Public License version 3,
//    or
// b) the Mozilla Public License Version 2.0.
//
// You should have received copies of the GNU General Public License and the
// Mozilla Public License along with this program. If not, see
// <https://www.gnu.org/licenses/> and <https://www.mozilla.org/MPL/2.0/>.

use crate::config;
use crate::config::{Issues, issue_path};

/// Largest weight of a harmonic, relative to the bass it is made from.
const MAX_WEIGHT: f64 = 10.0;
/// Largest level of the harmonics, relative to the bass they are made from.
const MAX_GAIN: f64 = 20.0;

/// Validate a VirtualBass config. Issue paths are relative to the parameters.
pub fn validate_config(
    samplerate: usize,
    conf: &config::VirtualBassParameters,
) -> Result<(), Issues> {
    let mut issues = Issues::new();
    if conf.low_freq() <= 0.0 {
        issues.invalid(issue_path!["low_freq"], "Low freq must be > 0");
    }
    if conf.high_freq() <= conf.low_freq() {
        issues.invalid(
            issue_path!["high_freq"],
            "High freq must be higher than low freq",
        );
    }
    if conf.harmonics_freq() >= samplerate as f64 / 2.0 {
        issues.invalid(
            issue_path!["harmonics_freq"],
            "Harmonics freq must be < samplerate/2",
        );
    } else if conf.harmonics_freq() <= conf.high_freq() {
        issues.invalid(
            issue_path!["harmonics_freq"],
            "Harmonics freq must be higher than high freq",
        );
    }
    for (name, weight) in [
        ("harmonic2", conf.harmonic2()),
        ("harmonic3", conf.harmonic3()),
        ("harmonic4", conf.harmonic4()),
    ] {
        if !(0.0..=MAX_WEIGHT).contains(&weight) {
            issues.invalid(
                issue_path![name],
                format!("Harmonic weight must be between 0 and {MAX_WEIGHT:.0}"),
            );
        }
    }
    if conf.gain() > MAX_GAIN {
        issues.invalid(
            issue_path!["gain"],
            format!("Gain cannot be larger than {MAX_GAIN:.0}"),
        );
    }
    if conf.attack() < 0.0 {
        issues.invalid(
            issue_path!["attack"],
            "Attack time must be greater than or equal to 0.",
        );
    }
    if conf.release() <= 0.0 {
        issues.invalid(
            issue_path!["release"],
            "Release time must be greater than 0.",
        );
    }
    issues.into_result(())
}

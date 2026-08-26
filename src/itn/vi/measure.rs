//! Measure tagger for Vietnamese.
//!
//! Converts spoken Vietnamese measurements to written form:
//! - "hai trăm ki lô mét" → "200 km"
//! - "hai trăm ki lô mét trên giờ" → "200 km/h"
//! - "năm mươi ki lô gam" → "50 kg"
//! - "hai mươi lăm độ xê" → "25 °C"
//! - "hai mét khối" → "2 m³"

use super::cardinal::words_to_number;

/// A measure unit definition: spoken Vietnamese forms (longest first) and the
/// written symbol.
struct Unit {
    words: &'static [&'static str],
    symbol: &'static str,
}

const UNITS: &[Unit] = &[
    // Compound rate units first (longest spoken form first).
    Unit {
        words: &["ki lô mét trên giờ", "ki lô mét một giờ"],
        symbol: "km/h",
    },
    Unit {
        words: &["mét trên giây"],
        symbol: "m/s",
    },
    // Plain units, longest spoken form first.
    Unit {
        words: &["ki lô mét vuông"],
        symbol: "km²",
    },
    Unit {
        words: &["ki lô gam trên mét khối"],
        symbol: "kg/m³",
    },
    Unit {
        words: &["ki lô gam"],
        symbol: "kg",
    },
    Unit {
        words: &["ki lô mét"],
        symbol: "km",
    },
    Unit {
        words: &["ki lô hét"],
        symbol: "kHz",
    },
    Unit {
        words: &["ki lô oát"],
        symbol: "kW",
    },
    Unit {
        words: &["ki lô vôn"],
        symbol: "kV",
    },
    Unit {
        words: &["ki lô jun"],
        symbol: "kJ",
    },
    Unit {
        words: &["ki lô pascal"],
        symbol: "kPa",
    },
    Unit {
        words: &["ki lô ca lo"],
        symbol: "kcal",
    },
    Unit {
        words: &["xăng ti mét"],
        symbol: "cm",
    },
    Unit {
        words: &["xăng ti mét khối"],
        symbol: "cm³",
    },
    Unit {
        words: &["xăng ti mét vuông"],
        symbol: "cm²",
    },
    Unit {
        words: &["mi li mét"],
        symbol: "mm",
    },
    Unit {
        words: &["mi li gam"],
        symbol: "mg",
    },
    Unit {
        words: &["mi li lít"],
        symbol: "ml",
    },
    Unit {
        words: &["mi li am pe"],
        symbol: "mA",
    },
    Unit {
        words: &["mê ga hét"],
        symbol: "MHz",
    },
    Unit {
        words: &["mê ga oát"],
        symbol: "MW",
    },
    Unit {
        words: &["mê ga pascal"],
        symbol: "MPa",
    },
    Unit {
        words: &["gi ga hét"],
        symbol: "GHz",
    },
    Unit {
        words: &["đê xi lít"],
        symbol: "dl",
    },
    Unit {
        words: &["đê xi bén"],
        symbol: "dB",
    },
    Unit {
        words: &["xen ti lít"],
        symbol: "cl",
    },
    Unit {
        words: &["mét khối"],
        symbol: "m³",
    },
    Unit {
        words: &["mét vuông"],
        symbol: "m²",
    },
    Unit {
        words: &["độ xê", "độ c", "độ xi"],
        symbol: "°C",
    },
    Unit {
        words: &["độ pha", "độ f"],
        symbol: "°F",
    },
    Unit {
        words: &["mét"],
        symbol: "m",
    },
    Unit {
        words: &["gam"],
        symbol: "g",
    },
    Unit {
        words: &["lít"],
        symbol: "l",
    },
    Unit {
        words: &["phút"],
        symbol: "min",
    },
    Unit {
        words: &["giây"],
        symbol: "s",
    },
    Unit {
        words: &["giờ"],
        symbol: "h",
    },
    Unit {
        words: &["hét"],
        symbol: "Hz",
    },
    Unit {
        words: &["oát"],
        symbol: "W",
    },
    Unit {
        words: &["vôn"],
        symbol: "V",
    },
    Unit {
        words: &["am pe"],
        symbol: "A",
    },
    Unit {
        words: &["ôm"],
        symbol: "Ω",
    },
    Unit {
        words: &["jun"],
        symbol: "J",
    },
    Unit {
        words: &["ca lo"],
        symbol: "cal",
    },
    Unit {
        words: &["pascal"],
        symbol: "Pa",
    },
    Unit {
        words: &["độ"],
        symbol: "°",
    },
];

/// Parse a spoken Vietnamese measurement expression to written form.
pub fn parse(input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();

    for unit in UNITS {
        for &word in unit.words {
            let suffix = format!(" {}", word);
            if let Some(num_part) = input_lower.strip_suffix(&suffix) {
                if let Some(num) = words_to_number(num_part) {
                    return Some(format!("{} {}", num, unit.symbol));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(parse("hai trăm ki lô mét"), Some("200 km".to_string()));
        assert_eq!(parse("năm mét"), Some("5 m".to_string()));
        assert_eq!(parse("mười xăng ti mét"), Some("10 cm".to_string()));
    }

    #[test]
    fn test_rate() {
        assert_eq!(
            parse("hai trăm ki lô mét trên giờ"),
            Some("200 km/h".to_string())
        );
        assert_eq!(parse("năm mét trên giây"), Some("5 m/s".to_string()));
    }

    #[test]
    fn test_weight() {
        assert_eq!(parse("năm mươi ki lô gam"), Some("50 kg".to_string()));
        assert_eq!(parse("chín mươi gam"), Some("90 g".to_string()));
    }

    #[test]
    fn test_temperature() {
        assert_eq!(parse("hai mươi lăm độ xê"), Some("25 °C".to_string()));
        assert_eq!(parse("một trăm độ"), Some("100 °".to_string()));
    }

    #[test]
    fn test_volume() {
        assert_eq!(parse("hai mét khối"), Some("2 m³".to_string()));
        assert_eq!(parse("năm mét vuông"), Some("5 m²".to_string()));
    }

    #[test]
    fn test_invalid() {
        assert_eq!(parse("hello"), None);
        assert_eq!(parse(""), None);
        assert_eq!(parse("ki lô mét"), None); // no amount
    }
}

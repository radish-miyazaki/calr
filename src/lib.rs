use std::str::FromStr;

use chrono::{Datelike, NaiveDate};
use clap::Parser;
use itertools::izip;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
struct Month {
    en: &'static str,
    ja: &'static str,
}

const MONTH_NAMES: [Month; 12] = [
    Month {
        en: "January",
        ja: "1月",
    },
    Month {
        en: "February",
        ja: "2月",
    },
    Month {
        en: "March",
        ja: "3月",
    },
    Month {
        en: "April",
        ja: "4月",
    },
    Month {
        en: "May",
        ja: "5月",
    },
    Month {
        en: "June",
        ja: "6月",
    },
    Month {
        en: "July",
        ja: "7月",
    },
    Month {
        en: "August",
        ja: "8月",
    },
    Month {
        en: "September",
        ja: "9月",
    },
    Month {
        en: "October",
        ja: "10月",
    },
    Month {
        en: "November",
        ja: "11月",
    },
    Month {
        en: "December",
        ja: "12月",
    },
];

fn parse_year(month: &str) -> Result<i32, String> {
    match i32::from_str(month) {
        Ok(v) => {
            if (1..=9999).contains(&v) {
                Ok(v)
            } else {
                Err(format!("year \"{}\" not in the range 1 through 9999", v))
            }
        }
        Err(_) => Err(format!("invalid year \"{}\"", month)),
    }
}

fn parse_month(month: &str) -> Result<u32, String> {
    match month.parse::<u32>() {
        Ok(v) => {
            if (1..=12).contains(&v) {
                Ok(v)
            } else {
                Err(format!("month \"{}\" not in the range 1 through 12", v))
            }
        }
        Err(_) => {
            let month = month.to_lowercase();
            let target_months = MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(i, m)| {
                    if m.en.to_lowercase().starts_with(&month) {
                        Some(i as u32 + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u32>>();

            if target_months.len() != 1 {
                return Err(format!("invalid month \"{}\"", month));
            }

            Ok(target_months[0])
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "calr",
    version = "0.1.0",
    author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>",
    about = "Rust cal"
)]
pub struct Args {
    #[arg(help = "Month name or number (1-12)", short, long, value_parser = parse_month)]
    month: Option<u32>,
    #[arg(help = "Year (1-9999)", value_name = "YEAR", value_parser = parse_year)]
    year: Option<i32>,
    #[arg(
        help = "Show whole current year",
        short = 'y',
        long = "year",
        default_value = "false",
        conflicts_with_all = ["year", "month"]
    )]
    show_current_year: bool,
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    let first_day_of_next_month = match NaiveDate::from_ymd_opt(year, month + 1, 1) {
        Some(d) => d,
        None => NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap(),
    };

    first_day_of_next_month.pred_opt().unwrap()
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let month_name = &MONTH_NAMES[(month - 1) as usize];
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day = last_day_in_month(year, month);
    let first_day_of_week = first_day.weekday().number_from_sunday() as usize;
    let last_day_of_week = last_day.weekday().number_from_sunday() as usize;

    let mut lines = vec![];
    // month and year header
    if print_year {
        lines.push(format!("{:>8} {:<13}", month_name.ja, year));
    } else {
        lines.push(format!("{:^20}  ", month_name.ja));
    }

    // day of week header
    lines.push("日 月 火 水 木 金 土  ".to_string());

    // days
    let mut line = " ".repeat(3 * (first_day_of_week - 1));
    for day in 1..=last_day.day() {
        let day_str = if today.day() == day && today.month() == month && today.year() == year {
            let style = ansi_term::Style::new();
            style.reverse().paint(format!("{:>2}", day)).to_string()
        } else {
            format!("{:>2}", day)
        };
        line.push_str(day_str.as_str());

        line.push(' ');
        if (first_day_of_week - 1 + day as usize) % 7 == 0 {
            line.push(' ');
            lines.push(line);
            line = String::new();
        }
    }
    if last_day_of_week != 7 {
        line.push_str(&" ".repeat(3 * (7 - last_day_of_week)));
        line.push(' ');
        lines.push(line);
    }

    for _ in 1..=8 - lines.len() {
        lines.push(" ".repeat(22));
    }

    lines
}

fn format_months_of_year(year: i32, today: NaiveDate) -> Vec<String> {
    let mut lines = vec![];

    lines.push(format!("{:>32}", year));
    for quater in 1..=4 {
        for (ms1, ms2, ms3) in izip!(
            format_month(year, (quater - 1) * 3 + 1, false, today),
            format_month(year, (quater - 1) * 3 + 2, false, today),
            format_month(year, (quater - 1) * 3 + 3, false, today)
        ) {
            lines.push(format!("{}{}{}", ms1, ms2, ms3));
        }

        if quater != 4 {
            lines.push("".to_string());
        }
    }

    lines
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let today = chrono::Local::now().date_naive();

    let lines = if args.show_current_year {
        let year = today.year();
        format_months_of_year(year, today)
    } else {
        match args.month {
            Some(month) => {
                let year = match args.year {
                    Some(year) => year,
                    None => today.year(),
                };
                format_month(year, month, true, today)
            }
            None => {
                if let Some(year) = args.year {
                    format_months_of_year(year, today)
                } else {
                    format_month(today.year(), today.month(), true, today)
                }
            }
        }
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{format_month, last_day_in_month, parse_month};

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1 through 12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1 through 12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "invalid month \"foo\"");
    }

    #[test]
    fn test_format_month() {
        let today = NaiveDate::from_ymd_opt(0, 1, 1).unwrap();
        let leap_feburary = vec![
            "      2月 2024         ",
            "日 月 火 水 木 金 土  ",
            "             1  2  3  ",
            " 4  5  6  7  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29        ",
            "                      ",
        ];
        assert_eq!(format_month(2024, 2, true, today), leap_feburary);

        let may = vec![
            "         5月           ",
            "日 月 火 水 木 金 土  ",
            "          1  2  3  4  ",
            " 5  6  7  8  9 10 11  ",
            "12 13 14 15 16 17 18  ",
            "19 20 21 22 23 24 25  ",
            "26 27 28 29 30 31     ",
            "                      ",
        ];
        assert_eq!(format_month(2024, 5, false, today), may);

        let today = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        let april = vec![
            "      4月 2024         ",
            "日 月 火 水 木 金 土  ",
            "   \u{1b}[7m 1\u{1b}[0m  2  3  4  5  6  ",
            " 7  8  9 10 11 12 13  ",
            "14 15 16 17 18 19 20  ",
            "21 22 23 24 25 26 27  ",
            "28 29 30              ",
            "                      ",
        ];
        assert_eq!(format_month(2024, 4, true, today), april);
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(
            last_day_in_month(2024, 1),
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()
        );
        assert_eq!(
            last_day_in_month(2024, 2),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );
        assert_eq!(
            last_day_in_month(2024, 4),
            NaiveDate::from_ymd_opt(2024, 4, 30).unwrap()
        )
    }
}

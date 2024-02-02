use chrono::{Datelike, Duration, NaiveDate, Utc};
use clap::Parser;
use std::error::Error;

use ansi_term::Colour;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    surname: String,
    #[serde(with = "my_date_format")]
    birthdate: Option<NaiveDate>,
}

mod my_date_format {
    use chrono::{Datelike, NaiveDate, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%d-%m-%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match !s.is_empty() {
            true => {
                let d = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                let year_now = Utc::now().naive_local().date().year();
                Ok(Some(NaiveDate::from_ymd(year_now, d.month(), d.day())))
            }
            false => Ok(None),
        }
    }
}

fn print_record(record: &Record) -> Result<(), &'static str> {
    let today = Utc::now().naive_local().date();
    let dur = Duration::days(2);
    let birthdate = match record.birthdate {
        Some(date) => date,
        None => return Err("Missing date error"),
    };

    let style = match birthdate - today <= dur {
        true => Colour::Red.bold(),
        false => Colour::Yellow.bold(),
    };

    println!(
        "{} {} {}.{}",
        record.name,
        record.surname,
        style.paint(birthdate.day().to_string()),
        style.paint(birthdate.month().to_string())
    );
    Ok(())
}

fn read_records(file: &str, days: i64) -> Result<Vec<Record>, csv::Error> {
    // let mut results = Vec::new();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path(file)?;
    let today = Utc::now().naive_local().date();
    let dur = Duration::days(days);

    let results: Result<Vec<_>, _> = rdr
        .deserialize()
        .into_iter()
        .filter_map(|res: Result<Record, _>| match res {
            Ok(record) => match record.birthdate {
                Some(date) => {
                    if (date - today <= dur) && (date - today).num_seconds() >= 0 {
                        Some(Ok(record))
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(e) => Some(Err(e)),
        })
        .collect();
    return results;
}

/// Prints the upcoming birthdays from the csv file.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// CSV file with columns name, surname, birthdate
    ///
    /// Note birthday like 31-01-2000 and ; as a delimiter
    #[clap(short, long)]
    birthday_file: String,

    /// Number of days to look ahead
    #[clap(short, long, default_value = "7")]
    days: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let conf = Args::parse();
    let mut records = read_records(&conf.birthday_file, conf.days)?;
    records.sort_by_key(|r| r.birthdate);
    if !records.is_empty() {
        for r in records {
            print_record(&r)?;
        }
    }
    return Ok(());
}

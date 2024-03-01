use chrono::NaiveDate;
use serde::de::{Deserializer, Deserialize};

#[inline]
fn remove_prefix(date_str: &str) -> &str {
    date_str.trim_start_matches("Reviewed ")
}

#[repr(u32)]
enum DirtyMonth {
    Jan = 1,
    Feb = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    Aug = 8,
    Sept = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

impl Into<u32> for DirtyMonth {
    fn into(self) -> u32 {
        self as u32
    }
}

impl std::str::FromStr for DirtyMonth {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Jan." => Ok(DirtyMonth::Jan),
            "Feb." => Ok(DirtyMonth::Feb),
            "March" => Ok(DirtyMonth::March),
            "April" => Ok(DirtyMonth::April),
            "May" => Ok(DirtyMonth::May),
            "June" => Ok(DirtyMonth::June),
            "July" => Ok(DirtyMonth::July),
            "Aug." => Ok(DirtyMonth::Aug),
            "Sept." => Ok(DirtyMonth::Sept),
            "Oct." => Ok(DirtyMonth::Oct),
            "Nov." => Ok(DirtyMonth::Nov),
            "Dec." => Ok(DirtyMonth::Dec),
            _ => Err("Invalid month"),
        }
    }
}

fn date_str_into_parts(date_str: &str) -> Result<(&str, &str, &str), &'static str> {
    let mut parts = date_str.split_whitespace();
    let month = parts.next().ok_or("No month")?;
    let day = parts.next().ok_or("No day")?;
    let year = parts.next().ok_or("No year")?;

    Ok((month, day, year))
}

#[inline]
fn parse_day_str(day_str: &str) -> Result<u32, &'static str> {
    day_str
        .trim_end_matches(',')
        .parse::<u32>()
        .map_err(|_| "Invalid day")
}

#[inline]
fn parse_year_str(year_str: &str) -> Result<i32, &'static str> {
    year_str
        .parse::<i32>()
        .map_err(|_| "Invalid year")
}

pub fn convert_date(date_str: &str) -> Result<NaiveDate, &'static str> {
    let date_str = date_str.trim_start_matches("Reviewed ");
    let (month_str, day_str, year_str) = date_str_into_parts(date_str)?;
    let month = month_str.parse::<DirtyMonth>()?.into();
    let day = parse_day_str(day_str)?;
    let year = parse_year_str(year_str)?;
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or("Invalid date")
}

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: &str = Deserialize::deserialize(deserializer)?;
    convert_date(date_str).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATE_1: &str = "Reviewed Sept. 13, 2023";
    const DATE_2: &str = "Reviewed June 13, 1999";
    const DATE_3: &str = "Reviewed Jan. 13, 2020";

    #[test]
    fn test_remove_prefix() {
        let date = remove_prefix(DATE_1);
        assert_eq!(date, "Sept. 13, 2023");
    }

    #[test]
    fn test_date_conversion() {
        let converted_date = convert_date(DATE_1).unwrap(); 

        assert_eq!(
            converted_date, 
            NaiveDate::from_ymd_opt(2023, 9, 13).unwrap()
        );

        let converted_date = convert_date(DATE_2).unwrap();

        assert_eq!(
            converted_date,
            NaiveDate::from_ymd_opt(1999, 6, 13).unwrap()
        );

        let converted_date = convert_date(DATE_3).unwrap();

        assert_eq!(
            converted_date,
            NaiveDate::from_ymd_opt(2020, 1, 13).unwrap()
        );
    }
}

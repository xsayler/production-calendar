use derive_builder::Builder;

#[derive(Debug, Clone, Copy)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl From<u8> for Month {
    fn from(value: u8) -> Self {
        match value {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("{} out of range: 1..12", value),
        }
    }
}

impl From<Month> for time::Month {
    fn from(value: Month) -> Self {
        match value {
            Month::January => time::Month::January,
            Month::February => time::Month::February,
            Month::March => time::Month::March,
            Month::April => time::Month::April,
            Month::May => time::Month::May,
            Month::June => time::Month::June,
            Month::July => time::Month::July,
            Month::August => time::Month::August,
            Month::September => time::Month::September,
            Month::October => time::Month::October,
            Month::November => time::Month::November,
            Month::December => time::Month::December,
        }
    }
}

impl From<time::Month> for Month {
    fn from(value: time::Month) -> Month {
        match value {
            time::Month::January => Month::January,
            time::Month::February => Month::February,
            time::Month::March => Month::March,
            time::Month::April => Month::April,
            time::Month::May => Month::May,
            time::Month::June => Month::June,
            time::Month::July => Month::July,
            time::Month::August => Month::August,
            time::Month::September => Month::September,
            time::Month::October => Month::October,
            time::Month::November => Month::November,
            time::Month::December => Month::December,
        }
    }
}

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(into))]
pub struct Day {
    pub date: time::Date,
    pub day: DayNumber,
    pub month: Month,
    pub year: i32,
    pub day_type: DayType,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum DayType {
    Working = 1,
    Weekend = 2,
    PreHoliday = 3,
    Holiday = 4,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct DayNumber {
    day: u8,
}

#[allow(dead_code)]
impl DayNumber {
    pub fn new(value: u8) -> DayNumber {
        match value {
            1..=31 => DayNumber { day: value },
            _ => panic!("{} out of range: 1..31", value),
        }
    }
}

impl From<u8> for DayNumber {
    fn from(value: u8) -> Self {
        DayNumber::new(value)
    }
}

impl From<DayNumber> for u8 {
    fn from(value: DayNumber) -> u8 {
        value.day
    }
}

impl PartialEq for DayNumber {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
    }
}

impl PartialOrd for DayNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.day.partial_cmp(&other.day)
    }
}

#[cfg(test)]
mod tests {
    use super::{DayNumber, Month};

    #[test]
    fn test_day_use_cases() {
        let day: DayNumber = 9.into();
        assert_eq!(day, DayNumber::new(9));
    }

    #[test]
    #[should_panic]
    fn test_day_new_less_1() {
        DayNumber::new(0);
    }

    #[test]
    #[should_panic]
    fn test_day_new_greater_31() {
        DayNumber::new(32);
    }

    #[test]
    fn test_day_new() {
        DayNumber::new(1);
        DayNumber::new(31);
    }

    #[test]
    fn test_day_into() {
        let day = DayNumber::new(15);

        let number: u8 = day.into();

        assert_eq!(15, number);
    }

    #[test]
    fn test_month_into() {
        let _: Month = 1.into();
        let _: Month = 12.into();
    }

    #[test]
    #[should_panic]
    fn test_month_into_lees_1() {
        let _: Month = 0.into();
    }

    #[test]
    #[should_panic]
    fn test_month_into_greater_12() {
        let _: Month = 13.into();
    }
}

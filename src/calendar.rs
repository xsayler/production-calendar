use crate::types::{Day, DayNumber, DayType, Month};
use std::error::Error;
use time::Date;

#[derive(Debug, Clone)]
pub struct ProductionCalendar {
    year: u32,
    days: Vec<Day>,
}

impl ProductionCalendar {
    pub fn new(year: u32, days: Vec<Day>) -> Self {
        ProductionCalendar { year, days }
    }

    pub fn get_year(&self) -> u32 {
        self.year
    }

    pub fn get_day(&self, date: Date) -> Result<&Day, Box<dyn Error>> {
        let days = self
            .days
            .iter()
            .filter(|d| d.date == date)
            .collect::<Vec<&Day>>();

        Ok(days.first().ok_or("Day not found")?)
    }

    pub fn get_days_count(&self) -> usize {
        self.days.len()
    }

    pub fn get_work_days_count(&self) -> usize {
        self.days.iter().filter(|day| day.day_type == DayType::Working || day.day_type == DayType::PreHoliday).count()
    }

    pub fn get_index(&self, date: Date) -> Result<usize, Box<dyn Error>> {
        self.days
            .iter()
            .position(|day| day.date == date)
            .ok_or("Index not found".into())
    }

    pub fn get_prev_work_day(&self, day: u8, month: Month) -> Result<&Day, Box<dyn Error>> {
        let date = Date::from_calendar_date(self.year.try_into()?, month.into(), day)?;
        let index = self.get_index(date)?;

        for day in self.days[1..=index].iter().rev() {
            if day.day_type == DayType::Working || day.day_type == DayType::PreHoliday {
                return Ok(day);
            }
        }

        Err(Box::<dyn Error>::from("Previos working day not found"))
    }

    pub fn count_work_days_in_month(&self, month: Month) -> Result<usize, Box<dyn Error>> {
        let count = self
            .days
            .iter()
            .filter(|day| {
                day.date.month() == month.into()
                    && (day.day_type == DayType::Working || day.day_type == DayType::PreHoliday)
            })
            .count();

        Ok(count)
    }

    pub fn get_days_in_month(&self, month: Month) -> Result<Vec<&Day>, Box<dyn Error>> {
        let days: Vec<&Day> = self
            .days
            .iter()
            .filter(|day| day.date.month() == month.into())
            .collect();

        Ok(days)
    }

    pub fn get_work_days_in_month(&self, month: Month) -> Result<Vec<&Day>, Box<dyn Error>> {
        let days: Vec<&Day> = self
            .days
            .iter()
            .filter(|day| {
                day.date.month() == month.into()
                    && (day.day_type == DayType::Working || day.day_type == DayType::PreHoliday)
            })
            .collect();

        Ok(days)
    }

    pub fn count_work_days_in_month_before(
        &self,
        month: Month,
        day: DayNumber,
    ) -> Result<usize, Box<dyn Error>> {
        let count = self
            .days
            .iter()
            .filter(|d| {
                d.date.month() == month.into()
                    && d.date.day() <= day.into()
                    && (d.day_type == DayType::Working || d.day_type == DayType::PreHoliday)
            })
            .count();

        Ok(count)
    }

    pub fn count_work_days_in_month_after(
        &self,
        month: Month,
        day: DayNumber,
    ) -> Result<usize, Box<dyn Error>> {
        let count = self
            .days
            .iter()
            .filter(|d| {
                d.date.month() == month.into()
                    && d.date.day() > day.into()
                    && (d.day_type == DayType::Working || d.day_type == DayType::PreHoliday)
            })
            .count();

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use time::{Date, Duration, Weekday};

    use crate::types::{DayBuilder, DayNumber, DayType, Month};

    use super::{Day, ProductionCalendar};

    fn get_day_type(date: Date) -> DayType {
        if date.month() == Month::January.into() && date.day() < 9 {
            return DayType::Holiday;
        }

        match date.weekday() {
            Weekday::Saturday | Weekday::Sunday => DayType::Weekend,
            _ => DayType::Working,
        }
    }

    fn make_calendar() -> ProductionCalendar {
        let mut date = Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2024, time::Month::December, 31).unwrap();

        let mut days: Vec<Day> = vec![];
        while date <= date_end {
            let day = DayBuilder::default()
                .year(2024)
                .day(DayNumber::new(date.day().into()))
                .month::<Month>(date.month().into())
                .date(date)
                .day_type(get_day_type(date))
                .build()
                .unwrap();

            days.push(day);

            date = date + Duration::days(1);
        }

        ProductionCalendar::new(2024, days)
    }

    #[test]
    fn test_make_calendar() {
        let calendar = make_calendar();

        assert_eq!(2024, calendar.get_year());
        assert_eq!(366, calendar.get_days_count());
    }

    #[test]
    fn test_get_index() {
        let calendar = make_calendar();

        assert_eq!(
            19,
            calendar
                .get_index(Date::from_calendar_date(2024, Month::January.into(), 20).unwrap())
                .unwrap()
        );
    }

    #[test]
    fn test_get_prev_work_day() {
        let calendar = make_calendar();

        assert_eq!(
            Date::from_calendar_date(2024, Month::January.into(), 19).unwrap(),
            calendar.get_prev_work_day(21, Month::January).unwrap().date
        );
    }

    #[test]
    fn test_count_work_days_in_month() {
        let calendar = make_calendar();

        assert_eq!(
            17,
            calendar.count_work_days_in_month(Month::January).unwrap()
        );
    }

    #[test]
    fn test_count_work_days_in_month_before() {
        let calendar = make_calendar();

        assert_eq!(
            5,
            calendar
                .count_work_days_in_month_before(Month::January, 15.into())
                .unwrap()
        );
    }

    #[test]
    fn test_work_days_in_month_after() {
        let calendar = make_calendar();

        assert_eq!(
            12,
            calendar
                .count_work_days_in_month_after(Month::January, 15.into())
                .unwrap()
        );
    }
}

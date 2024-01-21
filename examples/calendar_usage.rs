use production_calendar::{
    calendar::ProductionCalendar,
    types::{Day, DayBuilder, DayNumber, DayType, Month},
};
use time::{Date, Duration, Weekday};

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

pub fn main() {
    let calendar_2024 = make_calendar();

    println!(
        "Working days in January: {}",
        calendar_2024
            .count_work_days_in_month(Month::January)
            .unwrap()
    );
    println!(
        "Working days in January up to and including the 15th of January: {}",
        calendar_2024
            .count_work_days_in_month_before(Month::January, 15.into())
            .unwrap()
    );
    println!(
        "Working days in January after January 15: {}",
        calendar_2024
            .count_work_days_in_month_after(Month::January, 15.into())
            .unwrap()
    );
}

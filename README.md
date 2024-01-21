<h1> production-calendar </h1>
<p> Library for work with production calendar </p>

## Overview

The library allows you to calculate the number of working days in a month, etc. The library does not contain methods of loading the calendar, for this you need to use third-party loaders.

## Usage

```rust
pub fn main() {
    let calendar_2024 = make_calendar();

    println!("Working days in January: {}", calendar_2024.count_work_days_in_month(Month::January).unwrap());
    println!("Working days in January up to and including the 15th of January: {}", calendar_2024.count_work_days_in_month_before(Month::January, 15.into()).unwrap());
    println!("Working days in January after January 15: {}", calendar_2024.count_work_days_in_month_after(Month::January, 15.into()).unwrap());
}
```

For a full example, see: [examples](https://github.com/xsayler/production-calendar/tree/main/examples/calendar_usage.rs)

## License

This project is licensed under [MIT license](https://github.com/xsayler/production-calendar/blob/main/LICENSE).

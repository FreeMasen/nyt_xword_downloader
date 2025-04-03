use time::{Date, Month, Weekday};

pub struct DateIter {
    pub next: Option<Date>,
    pub end: Date,
    pub skip_sunday: bool,
}

impl DateIter {
    pub fn new(start: Date, end: Date) -> Self {
        Self {
            next: Some(start),
            end,
            skip_sunday: false,
        }
    }
    pub fn skip_sunday(&mut self) {
        let next_is_sunday = self.next.map(|n| n.weekday() == Weekday::Sunday);
        if next_is_sunday.unwrap_or_default() {
            self.next();
        }
        self.skip_sunday = true;
    }
}

impl Iterator for DateIter {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next?;
        if ret >= self.end {
            return None;
        }
        let mut new_next = ret;
        loop {
            let Some(next) = new_next.next_day() else {
                self.next = None;
                break;
            };
            new_next = next;
            if self.skip_sunday && new_next.weekday() == Weekday::Sunday {
                continue;
            }
            self.next = Some(new_next);
            break;
        }
        Some(ret)
    }
}

pub fn month_str(month: Month) -> &'static str {
    match month {
        Month::January => "Jan",
        Month::February => "Feb",
        Month::March => "Mar",
        Month::April => "Apr",
        Month::May => "May",
        Month::June => "Jun",
        Month::July => "Jul",
        Month::August => "Aug",
        Month::September => "Sep",
        Month::October => "Oct",
        Month::November => "Nov",
        Month::December => "Dec",
    }
}

pub fn parse_date(v: &str) -> Result<Date, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let dt = Date::parse(v, time::macros::format_description!("[year]-[month]-[day]"))?;
    if dt < Date::from_calendar_date(2011, Month::April, 1).unwrap() {
        return Err(format!(
            "Invalid date {} is before 2011-04-01",
            dt.format(time::macros::format_description!("[year]-[month]-[day]"))
                .unwrap()
        )
        .into());
    }
    Ok(dt)
}

#[cfg(test)]
mod tests {
    use time::Month;

    use super::*;

    #[test]
    fn date_iter_works_with_sundays() {
        let mut current = Date::from_calendar_date(2001, Month::January, 1).unwrap();
        let iter = DateIter {
            next: Some(current),
            end: Date::from_calendar_date(2002, Month::January, 1).unwrap(),
            skip_sunday: false,
        };
        for dt in iter {
            assert_eq!(dt, current);
            current = current.next_day().unwrap();
        }
    }

    #[test]
    fn date_iter_works_no_sundays() {
        let mut current = Date::from_calendar_date(1970, Month::January, 1).unwrap();
        let iter = DateIter {
            next: Some(current),
            end: Date::from_calendar_date(2024, Month::January, 1).unwrap(),
            skip_sunday: false,
        };
        for dt in iter {
            assert_eq!(dt, current);
            current = current.next_day().unwrap();
            if current.weekday() == Weekday::Sunday {
                current = current.next_day().unwrap();
            }
        }
    }
}

use chrono::{Utc, NaiveDate, Datelike, Weekday};
use chrono::prelude::Month;
use num_traits::FromPrimitive;
use colored::Colorize;

struct Calendar {
    year: i32,
    month_name: String,
    today: u32,
    days_matrix: Vec<Vec<u8>>
}

impl Calendar {
    fn create_month(year: i32, month: u32, today: u32) -> Option<Self> {
        let naive_date = NaiveDate::from_ymd_opt(year, month, 1);
        if naive_date.is_none() {
            return None;
        }

        let last_day_of_month = Self::get_last_day_in_month(year, month)
            .try_into()
            .unwrap();

        let mut day_iter = naive_date.unwrap().iter_days().enumerate();
        let mut weeks: Vec<Vec<u8>> = Vec::new();
        weeks.push(vec![0; 7]);

        loop {
            let (index, day) = day_iter.next().unwrap();
            let weekday = day.weekday().number_from_monday() - 1;

            let current_week = weeks.len() - 1;
            weeks[current_week][weekday as usize] = (index + 1) as u8;

            if weekday % 7 == 6 || index + 1 == last_day_of_month {
                if index + 1 < last_day_of_month {
                    weeks.push(vec![0; 7]);
                }
            }

            if index + 1 >= last_day_of_month {
                break;
            }
        }

        Some(Self {
            year,
            month_name: Month::from_u32(month).unwrap().name().to_owned(),
            today,
            days_matrix: weeks
        })
    }

    fn get_last_day_in_month(year: i32, month: u32) -> u32 {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1)
                       .unwrap())
            .pred_opt()
            .unwrap()
            .day()
    }

    fn pretty_print(&self) {
        let width = 28;
        let title = format!("{} - {}", self.month_name, self.year);
        println!();
        println!("{: ^width$}", title);
        println!();
        for i in 0..7 {
            print!("{} ", Weekday::from_u32(i).unwrap());
        }
        println!();

        for week in self.days_matrix.iter() {
            for day in week.iter() {
                if *day == 0 {
                    print!("    ");
                } else {
                    let day_format = format!(" {: >2} ", &day.to_string());
                    if *day == self.today as u8 {
                        print!("{}", day_format.black().on_white());
                    } else {
                        print!("{}", day_format);
                    }
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let today = now.day();

    let c = Calendar::create_month(year, month, today).unwrap();
    c.pretty_print();
}

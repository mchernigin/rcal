use chrono::{self, Datelike, DateTime, Local};
use colored::Colorize;

fn main() {
    let now = chrono::Local::now();

    print_month(now, now);
}

fn print_month(date: DateTime<Local>, now: DateTime<Local>) -> () {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let cur_month = months[date.month0() as usize];
    let cur_year = date.year();
    let month_and_year = format!("{} {}", cur_month, cur_year);
    println!("{:^20}", month_and_year);

    for day in ["Mo ", "Tu ", "We ", "Th ", "Fr ", "Sa ", "Su"] {
        print!("{}", day);
    }
    println!();

    let shift = date.with_day(1).unwrap().weekday() as u32;
    let days_in_month = days_in_month(date.month0(), date.year() as u32);
    print!("{:<1$}", "", (shift * 3) as usize);
    for day in 1..(days_in_month + 1) {
        let i =  (day + shift) % 7;
        let cell = format!("{day:>2}");
        if date.with_day(day).unwrap() == now {
            print!("{}", cell.reversed());
        } else {
            print!("{}", cell);
        }
        print!("{}", if i % 7 == 0 { "\n" } else { " " });
    }
    println!()
}

fn days_in_month(month: u32, year: u32) -> u32 {
    match month {
        1 => if is_leap(year) { 29 } else { 28 },
        3 | 5 | 8 | 10 => 30,
        _ => 31
    }
}

fn is_leap(year: u32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}
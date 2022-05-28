use std;
use clap::Parser;
use chrono::{self, Datelike, DateTime, Local};
use colored::Colorize;

/// Simple calendar
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Show weeks numbers
    #[clap(short, long)]
    week_number: bool,

    /// Show specific month of current year
    #[clap(short, long)]
    month: Option<u32>,
}

fn main() {
    let now = chrono::Local::now();
    let mut date = now;
    let args = Args::parse();

    match args.month {
        Some(m) =>
            match date.with_month(m) {
                Some(d) => date = d,
                None => die(format!("Invalid month: {}", m)),
            },
        None => (),
    }
    print_month(date, now, args.week_number)
}

fn print_month(date: DateTime<Local>, now: DateTime<Local>, show_week: bool) {
    let week_col_size = 4;
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
    if show_week {
        print!("{:<1$}", "", week_col_size);
    }
    println!("{:^20}", month_and_year);

    if show_week {
        print!("{:<1$}", "", week_col_size);
    }
    for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
        print!("{} ", day);
    }
    println!();

    let shift = date.with_day(1).unwrap().weekday() as u32;
    let days_in_month = days_in_month(date.month0(), date.year() as u32);
    let mut week_number = date.with_day(1).unwrap().iso_week().week();
    if show_week {
        print!("{week_number:>2}{:<1$}", "", week_col_size - 2);
    }
    print!("{:<1$}", "", (shift * 3) as usize);

    for day in 1..(days_in_month + 1) {
        let i =  (day + shift) % 7;
        let cell = format!("{day:>2}");

        if date.with_day(day).unwrap() == now {
            print!("{}", cell.reversed());
        } else {
            print!("{}", cell);
        }

        if i % 7 == 0 {
            println!();

            if show_week {
                week_number = (week_number + 1) % 52;
                print!("{week_number:>2}{:<1$}", "", week_col_size - 2);
            }
        } else {
            print!(" ");
        }
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

fn die(msg: String) {
    eprintln!("Error: {}", msg);
    std::process::exit(1);
}
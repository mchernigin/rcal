use std::{self, io::stdout};
use clap::Parser;
use chrono::{self, Datelike, Date, Local};
use crossterm::{cursor, execute, style::Attribute};

/// Simple calendar
#[derive(Parser, Debug)]
struct Args {
  /// Show weeks numbers
  #[clap(short, long)]
  week_number: bool,

  /// Also show previous and next months
  #[clap(short = '3')]
  three_months: bool,

  /// Show full year
  #[clap(short, long)]
  full_year: bool,

  /// Show specific month of current year
  #[clap(short, long)]
  month: Option<u32>,

  /// Show specific year
  #[clap(short, long)]
  year: Option<i32>,
}

fn main() {
  let now = chrono::Local::now().date();
  let mut date = now.with_day(1).unwrap();
  let month_width = 22;

  let args = Args::parse();
  match args.month {
    Some(m) =>
      match date.with_month(m) {
        Some(d) => date = d,
        None => {
          eprintln!("Invalid month: {}", m);
          std::process::exit(1);
        },
      },
    None => (),
  }
  match args.year {
    Some(y) =>
      match date.with_year(y) {
        Some(d) => date = d,
        None => {
          eprintln!("Invalid year: {}", y);
          std::process::exit(1);
        },
      },
    None => (),
  }
  
  if args.full_year || (args.year.is_some() && args.month.is_none()) {
    print_full_year(date, now, &args, month_width);
  } else if args.three_months {
    print_3months(date, now, &args, month_width);
  } else {
    print_month(date, now, &args, 0);
  }
}

fn print_full_year(date: Date<Local>, now: Date<Local>, cfg: &Args, w: u16) {
  for i in (2..12).step_by(3) {
    print_3months(date.with_month(i).unwrap(), now, &cfg, w);
    if i != 11 {
      println!();
    }
  }
}

fn print_3months(cur_month: Date<Local>, now: Date<Local>, cfg: &Args, w: u16) {
  let m = cur_month.month();

  let prev_month = if m == 1 {
    cur_month.with_month(12).unwrap()
             .with_year(cur_month.year() - 1)
  } else {
    cur_month.with_month(cur_month.month() - 1)
  }.unwrap();

  let next_month = if m == 12 {
    cur_month.with_month(1).unwrap().with_year(cur_month.year() + 1)
  } else {
    cur_month.with_month(cur_month.month() + 1)
  }.unwrap();

  let mut shift = 0;
  for month in [prev_month, cur_month, next_month] {
    print_month(month, now, &cfg, shift);
    if month != next_month {
      execute!(stdout(), cursor::MoveUp(month_height(month))).unwrap();
      shift += w;
    }
  }

  // Place cursor after longest month
  let max_h = std::cmp::max(month_height(prev_month),
                            std::cmp::max(month_height(cur_month),
                                          month_height(next_month)));
  let dy = max_h - month_height(next_month);
  execute!(stdout(), cursor::MoveDown(dy)).unwrap();
}

fn print_month(date: Date<Local>, now: Date<Local>, cfg: &Args, x: u16) {
  let week_col_size = if cfg.week_number { 4 } else { 0 };
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
  execute!(stdout(), cursor::MoveRight(x)).unwrap();
  print!("{:<1$}", "", week_col_size);
  println!("{:^20}", month_and_year);
  execute!(stdout(), cursor::MoveRight(x)).unwrap();

  print!("{:<1$}", "", week_col_size);
  for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
    print!("{} ", day);
  }
  println!();
  execute!(stdout(), cursor::MoveRight(x)).unwrap();

  let shift = date.with_day(1).unwrap().weekday() as u32;
  let days_in_month = days_in_month(date.month0(), date.year());
  let mut week_number = date.with_day(1).unwrap().iso_week().week();
  if cfg.week_number {
    print!("{week_number:>2}{:<1$}", "", week_col_size - 2);
  }
  print!("{:<1$}", "", (shift * 3) as usize);
  for day in 1..(days_in_month + 1) {
    let i =  (day + shift) % 7;
    let cell = format!("{day:>2}");

    if date.with_day(day).unwrap() == now {
      print!("{}{}{}", Attribute::Reverse, cell, Attribute::Reset);
    } else {
      print!("{}", cell);
    }

    if i % 7 == 0 {
      if day != days_in_month {
        println!();
      }
      execute!(stdout(), cursor::MoveRight(x)).unwrap();
      week_number = (week_number + 1) % 52;
      if cfg.week_number {
        print!("{week_number:>2}{:<1$}", "", week_col_size - 2);
      }
    } else {
      print!(" ");
    }
  }
  println!()
}

fn days_in_month(month: u32, year: i32) -> u32 {
  match month {
    1 => if is_leap(year) { 29 } else { 28 },
    3 | 5 | 8 | 10 => 30,
    _ => 31
  }
}

fn is_leap(year: i32) -> bool {
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

fn month_height(date: Date<Local>) -> u16 {
  if date.month() == 1 {
    return 8;
  }

  let last_day = days_in_month(date.month0(), date.year());
  let max_week = date.with_day(last_day).unwrap().iso_week().week();
  let min_week = date.with_day(1).unwrap().iso_week().week();
  let height = 3 + (max_week - min_week);
  height as u16
}

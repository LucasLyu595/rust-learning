extern crate chrono;
use chrono::*;

extern crate regex;
use regex::Regex;

pub fn run() {
    let re = Regex::new(r"(\d{2}):(\d+)").unwrap();
    println!("{:?}", re.captures("  10:230"));
    println!("{:?}", re.captures("[22:2]"));
    println!("{:?}", re.captures("10:x23"));
    let re = Regex::new(
        r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
",
    )
    .expect("bad regex");
    let caps = re.captures("2010-03-14").expect("match failed");

    assert_eq!("2010", &caps["year"]);
    assert_eq!("03", &caps["month"]);
    assert_eq!("14", &caps["day"]);

    let date = Local.ymd(2010, 3, 14);
    println!("date was {}", date);
    let date = Local.ymd_opt(2010, 3, 14);
    println!("date was {:?}", date);
    let date = Local.ymd_opt(2014, 24, 52);
    println!("date was {:?}", date);
}

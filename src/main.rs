use chrono::{DateTime, Datelike, Local, Timelike, Weekday};

fn weekday_sym(date: &DateTime<Local>) -> char {
    match date.weekday() {
        Weekday::Mon => '月',
        Weekday::Tue => '火',
        Weekday::Wed => '水',
        Weekday::Thu => '木',
        Weekday::Fri => '金',
        Weekday::Sat => '土',
        Weekday::Sun => '日',
    }
}

fn main() {
    let now = Local::now();

    // Calculate kiloseconds of the day.
    let tod = now.time();

    let secs = (tod.hour() * 3600) + tod.minute() * 60 + tod.second();

    let kilosecs = secs / 1000;
    let secs = secs % 1000;

    let secs_padding = if secs >= 100 {
        "".to_string()
    } else if secs >= 10 {
        "0".to_string()
    } else {
        "00".to_string()
    };

    println!(
        "{}.{}.{} ({}) - {}.{}{}",
        now.day(),
        now.month(),
        now.year(),
        weekday_sym(&now),
        kilosecs,
        secs_padding,
        secs
    );
}

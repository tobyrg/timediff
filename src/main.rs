use std::env;

fn main() {
    let start_time = env::args().nth(1).expect("Need a start time!");
    let end_time = env::args().nth(2).expect("Need a end time!");

    let parsed_start_time = parse_time(&start_time);
    let parsed_end_time = parse_time(&end_time);

    let mut diff = parsed_end_time - parsed_start_time;

    if diff < 0 {
        diff = diff + 1440;
    }

    let minutes = diff % 60;
    let hours = diff / 60;

    println!("{} Hours & {} Minutes", hours, minutes);
}

fn parse_time(time: &str) -> i32 {
    if time.contains("am") {
        let strip_time = time.replace("am", "");
        if strip_time.contains(":") {
            if strip_time.contains("12") {
                let total_minutes = parse_hour_minutes(&strip_time);
                return total_minutes - 720;
            }
            return parse_hour_minutes(&strip_time);
        } else {
            if strip_time.contains("12") {
                let total_minutes = 60 * strip_time.parse::<i32>().expect("Invalid number");
                return total_minutes - 720;
            }
            let total_minutes = 60 * strip_time.parse::<i32>().expect("Invalid number");
            return total_minutes;
        }
    }

    if time.contains("pm") {
        let strip_time = time.replace("pm", "");
        if strip_time.contains(":") {
            if strip_time.contains("12") {
                return parse_hour_minutes(&strip_time);
            }
            return parse_hour_minutes(&strip_time) + 720;
        } else {
            if strip_time.contains("12") {
                let total_minutes = 60 * strip_time.parse::<i32>().expect("Invalid number");
                return total_minutes;
            }
            let total_minutes = 60 * strip_time.parse::<i32>().expect("Invalid number");
            return total_minutes + 720;
        }
    }

    1
}

fn parse_hour_minutes(time: &str) -> i32 {
    let am_parts: Vec<&str> = time.split(":").collect();
    let pm_hour = am_parts[0].parse::<i32>().expect("Invalid Hour");
    let am_minutes = am_parts[1].parse::<i32>().expect("Invalid Hour");

    let total_minutes = pm_hour * 60 + am_minutes;
    return total_minutes;
}

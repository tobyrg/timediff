use std::env;

fn main() {
    let start_time = env::args().nth(1).expect("Need a start time!");
    let end_time = env::args().nth(2).expect("Need a end time!");
    let parsed_start_time = parse_time(&start_time);
    let parsed_end_time = parse_time(&end_time);
    let mut diff = parsed_end_time - parsed_start_time;
    if diff < 0 {
        diff = diff + 24;
    }
    println!("{} hours", diff)
}

fn parse_time(time: &str) -> i32 {
    if time.contains("am") == true {
        let strip_time = time.replace("am", "");
        let parsed_time = strip_time.parse::<i32>().expect("Invalid number");
        return parsed_time;
    }

    if time.contains("pm") == true {
        let strip_time = time.replace("pm", "");
        let parsed_time = strip_time.parse::<i32>().expect("Invalid number");
        if parsed_time == 12 {
            return parsed_time;
        }
        return parsed_time + 12;
    }

    0
}

use std::ops::Sub;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() -> Result<(), &'static str> {
	let args = std::env::args().collect::<Vec<String>>();
	if args.len() < 2 {
		return Err("Error: Invalid argument count.");
	}
	if (args[1] != "stopwatch" && args[1] != "timer")
		|| (args[1] == "timer" && args.len() < 3)
		|| (args[1] == "timer" && parse_time(args[2].clone()).is_err())
	{
		return Err("Error: Invalid arguments");
	}

	if args[1] == "stopwatch" {
		Ok(stopwatch())
	} else if args[1] == "timer" {
		timer(parse_time(args[2].clone())?)
	} else {
		Err("Error: Unknown Error (How did this even happen?)")
	}
}

fn stopwatch() {
	let start_time = SystemTime::now();
	let delay = Duration::from_millis(16); // around 60fps
	print!("\x1b[s"); // Save cursor position
	loop {
		print!("\x1b[u\x1b[0J"); // Restore cursor and clear
		println!("time: {}", format_duration(start_time.elapsed().unwrap()));
		thread::sleep(delay);
	}
}

fn timer(duration: Duration) -> Result<(), &'static str> {
	let delay = Duration::from_millis(16); // around 60fps
	if duration.is_zero() {
		return Err("Time is zero");
	}
	let start_time = SystemTime::now();
	print!("\x1b[s"); // Save cursor position
	while start_time.elapsed().unwrap() < duration {
		print!("\x1b[u\x1b[0J"); // Restore cursor and clear
		println!(
			"time: {}",
			format_duration(duration.sub(start_time.elapsed().unwrap()))
		);
		thread::sleep(delay);
	}
	Ok(())
}

fn parse_time(input: String) -> Result<Duration, &'static str> {
	let mut total_duration = Duration::ZERO;
	let mut current_number = String::new();

	let chars: Vec<char> = input.chars().collect();
	let mut i = 0;

	while i < chars.len() {
		if chars[i].is_ascii_digit() {
			current_number.push(chars[i]);
		} else if chars[i].is_alphabetic() {
			if current_number.is_empty() {
				return Err("Number expected before unit");
			}

			let value: u64 = current_number.parse().map_err(|_| "Invalid number")?;

			let unit = if i + 1 < chars.len() && chars[i + 1].is_alphabetic() {
				let two_char = format!("{}{}", chars[i], chars[i + 1]);
				i += 1;
				two_char
			} else {
				chars[i].to_string()
			};

			match unit.as_str() {
				"h" => total_duration += Duration::from_secs(value * 3600),
				"min" | "m" => total_duration += Duration::from_secs(value * 60),
				"s" => total_duration += Duration::from_secs(value),
				"ms" => total_duration += Duration::from_millis(value),
				"µs" | "us" => total_duration += Duration::from_micros(value),
				"ns" => total_duration += Duration::from_nanos(value),
				_ => return Err("Unknown time unit"),
			}

			current_number.clear();
		}
		i += 1;
	}

	Ok(total_duration)
}

fn format_duration(duration: Duration) -> String {
	let mut total_ns = duration.as_nanos();
	let hours = total_ns / 3_600_000_000_000;
	total_ns -= hours * 3_600_000_000_000;
	let minutes = total_ns / 60_000_000_000;
	total_ns -= minutes * 60_000_000_000;
	let seconds = total_ns / 1_000_000_000;
	total_ns -= seconds * 1_000_000_000;
	let milliseconds = total_ns / 1_000_000;
	total_ns -= milliseconds * 1_000_000;
	let microseconds = total_ns / 1_000;
	total_ns -= microseconds * 1_000;
	let nanoseconds = total_ns;

	let mut parts = Vec::new();

	if hours > 0 {
		parts.push(format!("{}h", hours));
	}
	if minutes > 0 || hours > 0 {
		parts.push(format!("{:02}m", minutes));
	}
	if seconds > 0 || minutes > 0 || hours > 0 {
		parts.push(format!("{:02}s", seconds));
	}
	if milliseconds > 0 || seconds > 0 || minutes > 0 || hours > 0 {
		parts.push(format!("{:03}ms", milliseconds));
	}
	if microseconds > 0 {
		parts.push(format!("{:03}µs", microseconds));
	}
	if nanoseconds > 0 {
		parts.push(format!("{:03}ns", nanoseconds));
	}

	if parts.is_empty() {
		"0s".to_string()
	} else {
		parts.join(" ")
	}
}

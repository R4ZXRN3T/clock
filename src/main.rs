//! A simple CLI clock application with stopwatch and timer modes.
//!
//! This application supports two modes:
//! - `stopwatch`: Counts up from zero
//! - `timer`: Counts down from a specified duration
//!
//! Time can be specified using units: h, min/m, s, ms, µs/us, ns

use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, queue};
use std::io::{stdout, Write};
use std::ops::Sub;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

/// Entry point for the clock application.
///
/// Parses command-line arguments to determine whether to run in stopwatch or timer mode.
///
/// # Arguments
/// - `stopwatch`: Runs the stopwatch
/// - `timer <duration>`: Runs a timer for the specified duration (e.g., "1h30m", "5m30s")
///
/// # Errors
/// Returns an error if arguments are invalid or the time format cannot be parsed.
fn main() -> Result<(), &'static str> {
	let args = std::env::args().collect::<Vec<String>>();
	if args.len() < 2 {
		println!("Error: No arguments provided.");
		display_information();
		return Err("Error: Invalid argument count.");
	}
	if (args[1] != "stopwatch" && args[1] != "timer")
		|| (args[1] == "timer" && args.len() < 3)
		|| (args[1] == "timer" && parse_time(args[2].clone()).is_err())
	{
		println!("Error: Invalid argument count.");
		display_information();
		return Err("Error: Invalid arguments");
	}

	if args[1] == "stopwatch" {
		Ok(stopwatch())
	} else if args[1] == "timer" {
		timer(parse_time(args[2].clone())?)
	} else {
		println!("Error: Unknown error (How did this even happen?)");
		display_information();
		Err("Error: Unknown Error (How did this even happen?)")
	}
}

/// Runs a stopwatch that counts up from zero.
///
/// Displays elapsed time continuously, updating at approximately 60 FPS.
/// The timer runs indefinitely until the process is terminated.
fn stopwatch() {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();
	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl-C handler");

	let running_input = running.clone();
	thread::spawn(move || {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).ok();
		running_input.store(false, Ordering::SeqCst);
	});

	let start_time = SystemTime::now();
	let delay = Duration::from_millis(16); // around 60fps
	println!("Press Ctrl-C or Enter to stop the stopwatch.");
	let mut stdout = stdout();
	execute!(stdout, cursor::SavePosition).unwrap();
	println!("Press Ctrl-C or Enter to stop the stopwatch.");

	while running.load(Ordering::SeqCst) {
		queue!(
			stdout,
			cursor::RestorePosition,
			Clear(ClearType::FromCursorDown)
		)
		.unwrap();
		queue!(
			stdout,
			Print(format!(
				"time: {}",
				format_duration(start_time.elapsed().unwrap())
			))
		)
		.unwrap();
		stdout.flush().unwrap();
		thread::sleep(delay);
	}
}

/// Runs a countdown timer for the specified duration.
///
/// # Arguments
/// * `duration` - The amount of time to count down from
///
/// # Returns
/// `Ok(())` when the timer completes successfully, or `Err` if the duration is zero
///
/// # Display
/// Updates at approximately 60 FPS, showing remaining time.
fn timer(duration: Duration) -> Result<(), &'static str> {
	// In stopwatch() and timer():
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();
	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl-C handler");

	let delay = Duration::from_millis(1); // around 60fps
	if duration.is_zero() {
		return Err("Time is zero");
	}
	let start_time = SystemTime::now();
	let mut stdout = stdout();
	execute!(stdout, cursor::SavePosition).unwrap();

	while running.load(Ordering::SeqCst) && start_time.elapsed().unwrap() < duration {
		queue!(
			stdout,
			cursor::RestorePosition,
			Clear(ClearType::FromCursorDown)
		)
		.unwrap();
		queue!(
			stdout,
			Print(format!(
				"time remaining: {}",
				format_duration(duration.sub(start_time.elapsed().unwrap()))
			))
		)
		.unwrap();
		stdout.flush().unwrap();
		thread::sleep(delay);
	}

	queue!(
		stdout,
		cursor::RestorePosition,
		Clear(ClearType::FromCursorDown)
	)
	.unwrap();
	queue!(stdout, Print("\n⏰ Timer finished!\n")).unwrap();
	queue!(stdout, Print('\x07')).unwrap();

	stdout.flush().unwrap();

	Ok(())
}

/// Parses a time string into a Duration.
///
/// Supports multiple time units in a single string:
/// - `h`: hours
/// - `min`, `m`: minutes
/// - `s`: seconds
/// - `ms`: milliseconds
/// - `µs`, `us`: microseconds
/// - `ns`: nanoseconds
///
/// # Arguments
/// * `input` - A time string (e.g., "1h30m45s", "5m", "500ms")
///
/// # Returns
/// `Ok(Duration)` with the total duration, or `Err` if the format is invalid
///
/// # Examples
/// ```
/// parse_time("5m30s".to_string()) // 330 seconds
/// parse_time("1h".to_string())    // 3600 seconds
/// ```
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

			// Handle both single and two-character units
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

/// Formats a Duration into a human-readable string.
///
/// Displays all non-zero time components, from largest to smallest.
/// Zero duration returns "0s".
///
/// # Arguments
/// * `duration` - The duration to format
///
/// # Returns
/// A formatted string representing the duration (e.g., "1h 30m 45s")
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

fn display_information() {
	println!("Usage:");
	println!("  clock stopwatch");
	println!("  clock timer <duration>");
	println!("\nExamples:");
	println!("  clock timer 5m 30s");
	println!("  clock timer 1h");
}

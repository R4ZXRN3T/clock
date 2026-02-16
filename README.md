# Clock - A Minimal Stopwatch & Timer

A lightweight, high-performance command-line stopwatch and timer utility written in Rust.

## Features

- **Stopwatch Mode**: Continuously count elapsed time from start
- **Timer Mode**: Count down from a specified duration
- **Flexible Time Input**: Support for multiple time units (hours, minutes, seconds, milliseconds, microseconds, nanoseconds)

## Usage

### Stopwatch

Start a stopwatch that counts up from zero:

```bash
./clock stopwatch
```

### Timer

Start a timer that counts down from a specified duration:

```bash
./clock timer <duration>
```

#### Duration Format

The duration supports various time units that can be combined:

- `h` - hours
- `m` or `min` - minutes
- `s` - seconds
- `ms` - milliseconds
- `µs` or `us` - microseconds
- `ns` - nanoseconds

#### Examples

```bash
./clock timer 5s            # 5 seconds
./clock timer 2min30s       # 2 minutes and 30 seconds
./clock timer 1h30min       # 1 hour and 30 minutes
./clock timer 500ms         # 500 milliseconds
./clock timer 1h 30min 45s  # 1 hour, 30 minutes, and 45 seconds
```

## Building

### Standard Build

```bash
cargo build --release
```

### Optimized Build

For minimal binary size, use the provided build script:

```bash
./build.sh
```

This script uses:
- Nightly Rust compiler features
- Custom `std` compilation with optimization for size
- Link-time optimization (LTO)
- UPX compression for ultra-compact binaries

The optimized binary is output to `./final/clock-0.1.0-Linux-x86_64/clock`

## Requirements

- Rust (for building)
- UPX (for ultra-compact binary compression, optional)

## Technical Details

- **Display Refresh**: ~60fps (16ms intervals) for smooth visual updates
- **Terminal Control**: Uses ANSI escape sequences for cursor positioning and clearing
- **Precision**: Displays time with nanosecond precision
- **Memory Efficient**: Minimal memory footprint with optimized build options

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](https://github.com/R4ZXRN3T/clock/blob/master/LICENSE) file for the full text.

You are free to use, modify, and distribute this software under the terms of the GPL-3.0. In summary:
- You may use this software for any purpose
- You must share modifications under the same GPL-3.0 license
- You must provide source code access to any modified versions
- You must include a copy of the license with distributions


# fitness-agent

A simple Rust program that gives detailed exercise plans based on user's input.

## Features

- Generates personalized exercise routines
- Takes user preferences (e.g., fitness goals, available equipment, schedule) into account
- Outputs detailed, actionable workout plans
- Command-line interface (CLI)
- [Add or modify features as your project evolves]

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version X.Y.Z or newer recommended)

### Building

```bash
git clone https://github.com/mohamadali-halwani/fitness-agent.git
cd fitness-agent
cargo build --release
```

### Running

```bash
cargo run -- [OPTIONS]
```

Example:

```bash
cargo run -- --goal strength --days 3 --equipment dumbbells
```

## Usage

Describe how users interact with your program. For example:

- Launch the CLI and answer the prompts
- Provide command-line arguments (see `--help` for details)

```bash
cargo run -- --help
```

## Example Output

```
Day 1:
- Warm-up: 5 min brisk walk
- Workout: 3x10 squats, 3x10 push-ups, 3x15 crunches
...
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Roadmap

- [ ] Add support for nutrition plans
- [ ] Integrate with wearable devices
- [ ] Export plans to PDF/CSV

## License

This project is licensed under the [GNU GPL v3](LICENSE).

## Contact

Created by [Mohamad Ali Halwani](https://github.com/mohamadali-halwani) – feel free to reach out!

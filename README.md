# Exercise Planner

This project provides a desktop application that generates daily exercise and diet plans using a generative AI backend. The user interface is built with `egui` via the `eframe` framework and features a translucent, morphic design.

## Download

```bash
git clone <repository-url>
cd fitness-agent
```

## Build

The application uses the Rust toolchain. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.

```bash
cargo build --release
```

## Install & Run

After building, run the application with:

```bash
cargo run --release
```

Before running, set your OpenAI API key in the environment:

```bash
export OPENAI_API_KEY="your-api-key"
```

This key is required to generate the personalized plans.

## Usage

Fill out your personal details in the application window and click **Generate Plan** to get a tailored workout and diet plan.

## License

This project is licensed under the terms of the GNU GPLv3. See [LICENSE](LICENSE) for details.


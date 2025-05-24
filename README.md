# Log Datapoint Extractor

This repository provides a Rust-based tool to efficiently parse large log files and extract unique datapoint names. The extracted datapoints are serialized into a readable JSON file.

## Features

- Efficient line-by-line processing for large log files
- Automatic extraction of unique datapoint names
- Serialization to a clean and readable JSON format

## Requirements

- Rust (stable release recommended)
- Cargo (included with Rust)

## Setup

1. **Clone the repository:**

```bash
git clone <repo-url>
```

2. **Build the project:**

```bash
cargo build --release
```

## Usage

Place your log file (`your_log_file.log`) into the repository directory, or specify a path.

Run the tool:

```bash
cargo run --release
```

### Output

The program generates a file named `unique_datapoints.json`, containing all unique datapoints extracted from the provided log file:

```json
{
	"unique_datapoints": ["cost_tracker_stats", "tower-observed"]
}
```

## Customizing Input and Output

To customize file names, simply edit the `main.rs` file:

```rust
let out_file = File::create("unique_datapoints.json")?; // Output file
```

## Dependencies

- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)

## License

This project is available under the License. See the [LICENSE](LICENSE) file for more details.

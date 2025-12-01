# Pelican Framework

Pelican is a high-performance data processing framework focused on capturing, processing, and analyzing network data streams. It provides a flexible component system that allows users to build complex data processing pipelines.

## Features

- **Modular Design**: Offers an extensible component system including data sources, processors, and sinks.
- **High Performance**: Leverages Rust’s performance advantages and asynchronous programming model to achieve efficient data processing.
- **Security**: Supports data encryption and integrity verification to ensure data security during processing.
- **Monitoring and Statistics**: Built-in metric collectors enable real-time monitoring of data processing performance and status.
- **Traffic Control**: Provides rate limiting and backpressure mechanisms to prevent system overload.

## Components

- **Sources**: Support in-memory data sources, duplicate data sources, and network packet capture sources.
- **Processors**: Include filter processors, case conversion processors, packet statistics processors, IP analysis processors, and threat detection processors.
- **Sinks**: Provide console output, statistics collection, and secure file storage capabilities.
- **Configuration Management**: Manages pipeline configuration through configuration structs.
- **Security Module**: Offers data encryption, decryption, and integrity verification functions.
- **Traffic Control**: Supports rate limiting and backpressure to ensure system stability and reliability.

## Examples

The project includes several example programs demonstrating how to build data processing pipelines using the Pelican framework:

- `basic.rs`: Basic example showing how to create a simple data processing pipeline.
- `benchmark.rs`: Performance benchmark example.
- `config_example.rs`: Configuration management example.
- `minimal.rs`: Minimal example showing a data processing pipeline with the simplest configuration.
- `network.rs`: Network packet capture and processing example.
- `performance.rs`: Performance optimization example.
- `secure_pipeline.rs`: Secure data processing pipeline example.

## Usage

1. Install the Rust toolchain.
2. Clone the repository to your local machine.
3. Build the project using Cargo.
4. Run the example programs to learn how to use the framework.

## Contribution

Contributions of code and documentation are welcome to help improve the Pelican framework. Please follow the project’s contribution guidelines when submitting changes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
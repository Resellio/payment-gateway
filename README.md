# Payment Gateway

Payment Gateway is a mock payment processing service designed to be used by the Resellio API. It simulates real-world payment gateway functionalities and is used for testing the Resellio ecosystem.

## Getting started

### Prerequisites

- rust version >= `1.85`
- docker version >= `28.0`

### Running locally

1. Clone the repository:

   ```bash
   git clone https://github.com/Resellio/payment-gateway.git
   cd payment-gateway
   ```

2. Create `.env` file and fill it accordingly to `.env.local`.

3. Build and start application:

   ```bash
   cargo run
   ```

   You can optionally pass `--release` flag to run optimized build:

   ```bash
   cargo run --release
   ```

### Running in docker

1. Clone the repository:

   ```bash
   git clone https://github.com/Resellio/payment-gateway.git
   cd payment-gateway
   ```

2. Build an image:

   ```bash
   docker build -t payment-gateway .
   ```

3. Run container:

   ```bash
   docker run -e API_ORIGIN={API_ORIGIN} -e PORT={APP_PORT} -p {YOUR_MACHINE_PORT}:{APP_PORT} payment-gateway
   ```

### Testing

1. Run tests using `cargo`:
   ```bash
   cargo test
   ```

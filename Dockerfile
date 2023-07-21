# Use a base image with Rust installed
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the necessary files into the container
COPY . .

# Install curl and other dependencies
RUN apt-get update && apt-get install -y curl

# Run the commands to install Rust, build the project, and run the server
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH="$HOME/.cargo/bin:$PATH" && \
    . ~/.bashrc && \
    cargo build --release

# Set the command to run the server
CMD ["./target/release/warp_page"]
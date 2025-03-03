# NOTE: git commands below generated with GPT-4o
# Use the official Rust image as the base image
# See tag options: https://hub.docker.com/_/rust/
FROM rust:1.85.0-slim-bookworm 

# Install git (remove extra files after installation)
RUN apt-get update && apt-get install -y git --no-install-recommends && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into WORKDIR
COPY . .

# Following build steps here: https://github.com/koala-planner/HDDL-Parser?tab=readme-ov-file#build-instruction
# 1. Met by base rust image above
# 2. Add required parts of CreuSAT:
#   - Clone the CreuSAT repository with minimal history and no file contents initially
#   - Checkout a specific commit (CREUSAT_COMMIT) to prevent breaking changes
#   - Configure sparse-checkout to only include the 'Robinson' directory to copy into WORKDIR
#   - Clean-up unnecessary repo files

# Checkout specific version to prevent breaking changes:
ENV CREUSAT_COMMIT="b36aacd53874c49e77493ac6ecceabf0b1968154"
RUN git clone --depth 1 --filter=blob:none --sparse https://github.com/sarsko/CreuSAT.git --branch master \
    && cd CreuSAT \
    && git reset --hard ${CREUSAT_COMMIT}\
    && git sparse-checkout set Robinson \
    && cp -r Robinson ../Robinson \
    && cd .. \
    && rm -rf CreuSAT

# 3. Build the project in release mode
#   The nightly build is required by build step (creusot-contracts-proc v0.2.0):
#   Pin Specific nightly build to allow CreuSAT/Robinson to build:
RUN rustup install nightly-2024-01-31 && rustup default nightly-2024-01-31

# Build the project in release mode, clean-up any files after build:
RUN cargo build --release

# Set the entrypoint to the built executable
ENTRYPOINT ["/app/target/release/hddl_analyzer"]
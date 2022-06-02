##################################
# STEP 1 build executable binary #
##################################
FROM rust:1.61.0-alpine3.15 AS builder

# Install git (required for fetching the dependencies)
RUN apk update && apk add --no-cache git

# Create appuser
ENV USER=next_boat
ENV UID=10001 
# See https://stackoverflow.com/a/55757473/12429735RUN 
RUN adduser \    
    --disabled-password \    
    --gecos "" \    
    --home "/nonexistent" \    
    --shell "/sbin/nologin" \    
    --no-create-home \    
    --uid "${UID}" \    
    "${USER}"

# Set up build directories
RUN mkdir -p /app && \
    mkdir -p /BUILD

WORKDIR /BUILD

COPY src /BUILD/src
COPY Cargo.toml /BUILD/Cargo.toml

# Fetch dependencies
RUN cargo clean
RUN cargo update

# Build the binary
RUN cargo build --release

##############################
# STEP 2 build a small image #
##############################
FROM scratch

# Import the user and group files from the builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy our static executable
COPY --from=builder /BUILD/target/release/next_boat /app

# Use an unprivileged user
USER next_boat:next_boat

EXPOSE 5001

# Run the binary
CMD ["/app/next_boat"]

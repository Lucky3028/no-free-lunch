FROM gcr.io/distroless/cc:latest
USER nonroot
WORKDIR /app
COPY ./target/release/no-free-lunch .

ENTRYPOINT ["/app/no-free-lunch"]
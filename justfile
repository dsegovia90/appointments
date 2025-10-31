test argument="":
  cargo nextest run --features mock-time --test-threads=1  --no-fail-fast -- {{argument}}

alias snap := snapshot-review
snapshot-review:
  cargo insta review # requires cargo-insta cli tool. https://insta.rs/docs/quickstart/#installation

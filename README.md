# rate-limit-testing

```$ rate-limit-testing <url> <delay-between-requests>```
#### A delay greater than 100 will be assumed to be in milliseconds
#### Delay less than 100 will be assumed as seconds

### example;
```$ cargo run -- https://koonts.net/ 0```

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check

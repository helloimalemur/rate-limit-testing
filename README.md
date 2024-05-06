# rate-limit-testing

## Install
```shell
$ cargo install rate-limit-testing
```

### GET request
```shell
$ rate-limit-testing -u <url> -d <delay-between-requests>
```

### POST request (automatically uses POST if payload/post_data is provided)
```shell
$ rate-limit-testing -u <url> -d <delay-between-requests> -p "the body"
```

### Headers
```shell
$ rate-limit-testing -u <url> -d <delay-between-requests> -p "the body" -a "HEADER1: value, HEADER2: value"
```


## Note;
#### A delay greater than 100 will be assumed to be in milliseconds
#### Delay less than 100 will be assumed as seconds

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check

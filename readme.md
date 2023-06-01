A little CLI tool to check the weather forecast.

## Install
Using https://github.com/rossmacarthur/install:
```
curl --proto '=https' --tlsv1.2 -sSf https://rossmacarthur.github.io/install/crate.sh \
    | bash -s -- --repo "alessioRaviola/circe" --to ~/.cargo/bin
```

## Use

Usage: `circe [OPTIONS] <LOCATION>`

Arguments:
- <LOCATION>  Name of the location to check the weather for

Options:
-  `-m <MAX_DATA>`      Maximum number of data points to show the forecast for [default: 12]
-  `-s`                 Only show time and forecast, no details
-  `-h, --help`         Print help

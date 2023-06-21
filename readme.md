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
  <LOCATION>  Name of the location to check the weather for

Options:
-  `-m <MAX_DATA>`      Maximum number of data points to show the forecast for [default: 12]
-  `-t`                 Show temperature data
-  `-p`                 Show precipitations data
-  `-u`                 Show humidity data
-  `-w`                 Show wind data
-  `--full`         Show full data, same as `-t -p -u -w`
-  `-h, --help`         Print help

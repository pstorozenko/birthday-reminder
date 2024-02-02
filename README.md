# Birthday reminder

A simple rust program to remind you of your friends' birthdays.
Ideally you want to run it on a terminal start.

## Usage

Please use the devcontainer for the environment.

```bash
cargo build
birthday-reminder --help
```

For example

```bash
birthday-reminder -b example_birthdays.csv -d 7
```

Note the semi-colon separated file format, and the date format:

```csv
name;surname;birthdate
John;Doe;31-01-2000
```

## Disclaimer

This project has been written ~5 years ago, and I'm not rust developer by any means.
Moving it to github for the sake of it.
I just slightly refactored it to use `clap` for argument parsing.
All other libraries could be unmaintained or outdated.

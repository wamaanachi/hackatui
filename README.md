# hackatui

A small terminal app for viewing your Hackatime coding stats.

It shows your all-time coding total, today's time, daily goal, projects, languages and editors. There are also weekly activity charts, an hourly activity grid, a monthly calendar with streaks, and project details and timelines.

Built with Rust, Ratatui and tui-piechart for the interface, Reqwest and Tokio for API requests, and Serde for reading JSON. It uses the Hackatime API for your data.

## Build and run

Install Rust and Cargo, then clone the repo:

```sh
git clone https://github.com/wamaanachi/hackatui.git
cd hackatui
```

Get your API key from [Hackatime settings](https://hackatime.hackclub.com/my/settings). If you already have an `api_key` in `~/.wakatime.cfg`, the app can read it there. Otherwise, enter it in Bash:

```bash
read -rsp "Hackatime API key: " HACKATIME_API_KEY; echo
export HACKATIME_API_KEY
```

Paste the key when the prompt appears and press Enter. The input stays hidden.

Run the app:

```sh
cargo run --release
```

To build it without starting it:

```sh
cargo build --release
```

The executable is `target/release/hackatui`. You need an internet connection to load your stats.

## Controls

- `1`: summary and today's time
- `2`: daily goal
- `3`: project durations
- `4`: weekly activity
- `5`: languages
- `6`: editors
- `7`: hourly activity
- `8`: project details, with left and right arrows to switch projects
- `9`: project cards
- `0`: monthly calendar
- `t`: project timeline
- `x`: quit

The total is all-time, while the top stats, project durations and pie charts cover the last seven days. Historical charts use UTC. Calendar streaks only cover the current month. Project cards and details use the loaded history range, not all-time totals.

Data loads when you start the app. Restart it to refresh. It displays activity already tracked by Hackatime; it does not track coding time itself.

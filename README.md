# hackatui

[![hackatui](https://img.shields.io/badge/GitHub-hackatui-green?style=plastic)](https://github.com/wamaanachi/hackatui)

> for all your prophesizing dreams

---

## Features

All the same features as the hackatime site

## Stack

*   **Frontend:**   Ratatui / Terminal
*   **Backend:**    Rust
*   **Api:**        Hackatime
*   **Styling:**    Vibez

---

## Getting Started

Follow these simple steps to get a local copy up and running.

### Prerequisites

None (Unless you want to build it from source)

### How to get this for yourselves:

**Download it**

Get the latest release from [GitHub](https://github.com/wamaanachi/hackatui/releases)

>**Note:** The name of the executable will be "hackatui-*", where * is the version number, and build version,
> remember to type the full name when executing like ```./hackatui-* -V``` , or rename it from "hackatui-*" to "hackatui"

> If you downloaded, most likely it is in the downloads directory,
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./hackatui```


Or download from command line, like this:


#### Download (Linux only)

```shell
curl -L https://github.com/wamaanachi/hackatui/releases/download/Release/hackatui-0.1.0-x86_64-Linux -o hackatui
chmod +x hackatui
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./hackatui -V
```

## Building from Source

(linux only)

1. **Pre-requisites:**  
   Need to install Rust


2. **Clone the repository:**
   ```shell
   git clone https://github.com/wamaanachi/hackatui.git
   cd hackatui
   ```


3. **Build**:
   Probably just run:

   ```shell
   cargo build --release
   ```


4. **Done!:**  
   Now test the binary with:
   ```shell
   ./hackatui -V
   ```

5. Add to alias:    
   If you want to add it so that you can run it directly without the './', then do this:

If using Bash:
```shell
mv hackatui .local/bin/
echo 'alias hackatui="./.local/bin/hackatui"' >> .bashrc
```
If using Zsh:
```shell
mv hackatui ~/local/bin/hackatui
echo 'alias hackatui="./.local/bin/hackatui"' >> .bashrc
```
If using Fish:
```shell
mv hackatui ~/local/bin/hackatui
abbr -a hackatui "./.local/bin/hackatui"
```

## Contributors
*   **[![Amaan](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
*   **[![Ruster](https://img.shields.io/badge/GitHub-Ruster-orange?style=plastic)](https://www.github.com/Tylerr-D)**
*   **[![Chish](https://img.shields.io/badge/GitHub-Chish-orange?style=plastic)](https://github.com/chishxd)**


### Credits
*  **[Hackatime](https://hackatime.hackclub.com)**
*  **[Wakatime](https://wakatime.com)**


> Coded and tested in Ubuntu, Fedora and Arch Linux, should work in any distro.

## Help
```shell
Usage: hackatui [OPTIONS]

Options:
  -u, --user                               Gives the current user
      --summary                            Gives a Summary of your hacking
      --status                             Gives Status for today
      --waka-stats                         Gives stats for the past week
      --stats                              Gives u stats
      --heartbeat <HEARTBEAT>              Heartbeat Details
      --project-details <PROJECT_DETAILS>  Gives details of your projects
      --currently-hacking                  How many people are currently hacking?
      --banned                             How many banned users?
      --leaderboard <LEADERBOARD>          Leaderboard Status
      --trust-value <TRUST>                Trust value of user [default: self]
      --badge <BADGE>                      Gives a shields.io badge uri for your project
      --lookup <LOOKUP>                    Lookup user
      --no-widget                          
  -h, --help                               Print help (see more with '--help')
  -V, --version                            Print version
```
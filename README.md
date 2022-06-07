# Plex Discord Rich Presence

Displays what movies or shows you're currently watching your Plex server.

Heavily inspired by [mpd-discord-rpc](https://github.com/JakeStanger/mpd-discord-rpc) by [Jake Stanger](https://github.com/JakeStanger).

## Installation

Install via cargo:

```bash
cargo install plex-discord-rpc
```

## Manual installation

Build a release binary using the following command:

```bash
cargo build --release --locked
```

Then copy it from `target/release/plex-discord-rpc` to your favorite `bin` folder, and make it executable.

```bash
cp target/release/plex-discord-rpc ~/.local/bin
chmod +x ~/.local/bin/plex-discord-rpc
```

Then, assuming your `bin` folder is in your `$PATH`, run the binary:

```bash
plex-discord-rpc
```

## Configuration

Running the program once will generate a default configuration file. On Linux, this will be at `~/.config/plex-discord-rpc/config.toml`.

### Plex Configuration
- `host` - The hostname of your Plex server. Example: `host.plex.local`
- `username` - Your username on the Plex server. This is used to track which session is yours and what you're currently watching.
- `token` - The `X-Plex-Token` for your Plex server. See [here](https://support.plex.tv/articles/204059436-finding-an-authentication-token-x-plex-token/) on how to find your token.
- `tls` - If your Plex server is using TLS (IE `https` or `wss`) then this should be set to `true`.

### Discord Configuration
- `app_id` - The Discord application ID to use for rich presence activity.
- `large_image` - The large image asset used for rich presence.
- `small_image` - The small image asset used for rich presence.
- `large_text` - The text that shows on hovering the large image.
- `small_text` - The text that shows on hovering the small image.

## Default configuration

```toml
[plex]
host = "localhost:32400"
username = "admin"
token = "change me"
tls = true

[discord]
app_id = 979815538509348874
large_image = "plex"
small_image = ""
large_text = "Plex"
small_text = ""

```
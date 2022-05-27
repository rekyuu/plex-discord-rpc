# Plex Discord Rich Presence

Displays what movies or shows you're currently watching your Plex server.

Note that this will **always** use SSL for both WebSocket and HTTP requests. Your server should be set up to support both `wss` and `https`.

Heavily inspired by [mpd-discord-rpc](https://github.com/JakeStanger/mpd-discord-rpc) by [Jake Stanger](https://github.com/JakeStanger).

## Building and installation

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

[discord]
app_id = 979815538509348874
large_image = "plex"
small_image = ""
large_text = "Plex"
small_text = ""

```
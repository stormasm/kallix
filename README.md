
So currently the *removeoverlay* branch is for updating the code to the latest
gpui code.  All that was added here was the ability to get the cmd-q working
again; as it broke when the code got updated to the latest gpui.

Next thing on the agenda is to get the overlay to anchored stuff working
in [context_menu](https://discord.com/channels/@me/1215009852296790017/1225893620415205437)

See also these notes: https://github.com/stormasm/zednotes/blob/main/overlay.md

### Start here, everything above this point is local notes of the day

This started out as _Kallax_ and then I ported it to _Kallex_ and now _Kallix_ is just a template with no functionality but gives the infrastructure of the tabs and the ability to have different views in each one.

- This is a fork of [kallax](https://github.com/Ben-Wormald/kallax)

### Youtube Downloaders

- Go to Youtube Studio and then Audio Library to download MP3 files
- https://github.com/ytdl-org/youtube-dl
- https://y2mate.nu/tuqU/

Download your music and put it inside your LIBRARY_DIR directory

## Development

Create a `.env` file in the repo root:

```sh
LIBRARY_DIR="/Users/ma/Music"
```

### Clear your cache

Tracks loaded from your library are cached - run

```rust
rm -rf "~/Library/Application Support/org.kallax.kallax"
```

to clear the cache and change your library directory.

The location of your database files are based on this code:

- https://github.com/trevyn/turbosql?tab=readme-ov-file#wheres-my-data

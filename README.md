This started out as _Kallax_ and then I ported it to _Kallex_ and now _Kallix_ is just a template with no functionality but gives the infrastructure of the tabs and the ability to have different views in each one.

- This is a fork of [kallax](https://github.com/Ben-Wormald/kallax)

### Youtube Downloaders

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

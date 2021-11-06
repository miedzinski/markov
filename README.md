# Markov

A Markov chain based Discord chat bot.

## Building

It is recommended to use [cargo](https://rustup.rs). To install:

```shell
cargo install --git https://github.com/miedzinski/markov.git
```

Check if it's successfully installed:

```shell
markov --help
```

## Usage

To run as a Discord bot you need an API token. To obtain one go to
[developer portal](https://discord.com/developers/applications/). Then run:

```shell
markov --token <YOUR TOKEN HERE>
```

## Persistent storage

By default, the bot stores entire Markov chain in the memory and doesn't persist
it to the disk. This can be mitigated by running with SQLite backend by
passing `--sqlite-path /path/to/sqlite.db` option. If it's the first time
running, run with `--setup-db` to create necessary tables.

## License

GNU GPLv3. See [LICENSE](LICENSE).

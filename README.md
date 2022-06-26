# 🦀 Anime Game CLI

CLI tool to manage your Anime Game installation

This is not and will not be the primary project, and most of the effort will always be given to other projects, including GUI launcher. That being said, this project is used more like a test site to test the [anime core library](https://gitlab.com/an-anime-team/anime-game-core)

## Roadmap to 1.0.0

| status | command | subcommand | description |
| :-: | - | - | - |
| ✅ | game | info | Get installed game info |
| ❌ | | download | Download the game |
| ❌ | | update | Update the game |
| ✅ | | repair | Repair the game |
| ✅ | voice | info | List installed voice packages |
| ❌ | | download | Install additional voice package |
| ❌ | | update | Update voice packages |
| ❌ | | remove | Remove voice package |
| ✅ | | repair | Repair voice packages |
| ✅ | patch | info | Get info about linux patch |
| ✅ | | sync | Sync latest patch from remote repo |
| ✅ | | apply | Apply patch |
| ✅ | | revert | Revert patch |
| ✅ | info | | Get info about game, patch and voice packages |
| ✅ | run | | Run the game |

## Mini-wiki

### game repair

| argument | description | example |
| - | - | - |
| `--threads (-t)` | Number of threads used to verify and repair files | `./anime-game-cli game repair -t 12` |
| `--verify-threads (-vt)` | Number of threads used to verify files | `./anime-game-cli game repair -vt 12` |
| `--repair-threads (-rt)` | Number of threads used to repair files | `./anime-game-cli game repair -rt 12` |
| `--ignore (-i, --skip)` | List of names repairer will avoid to repair files with | `./anime-game-cli game repair --ignore='unity,xdelta,report'` |
| `--verify (-v` | Don't repair files and just print broken ones | `./anime-game-cli game repair -v` |

> Note that more verification threads means more memory consumption as they need to store and calculate huge binaries' hashes
>
> That being said, 16 threads can consume up to 1.3 GB of RAM

### voice repair

```
./anime-game-cli voice repair [packages names] [arguments]
```

Example:

```
./anime-game-cli voice repair english japanese -v
```

Uses the same arguments as `game repair`

### run

Example `config.toml` file:

```toml
[paths]
game = "/path/to/Genshin Impact" # Path to the game folder

[patch]
hosts = ["https://path-to.com/linux/patch"] # Linux patch hosts

[wine]
prefix = "/path/to/lutris-GE-Proton7-16-x86_64" # Wine prefix
executable = "/path/to/lutris-GE-Proton7-16-x86_64/bin/wine64" # Wine executable

# Environment variables
[wine.environment]
LANG = "ru_RU.UTF8"
```

### patch sync

Syncs linux patch with remote repository

| argument | description | example |
| - | - | - |
| `--recursive (-r)` | Try each provided repository untill it's succeeded. Otherwise will be used the first one | `./anime-game-cli patch sync -r` |

### patch revert

Revert linux patch from the game

| argument | description | example |
| - | - | - |
| `--force (-f)` | Avoid checking of the files timestamps | `./anime-game-cli patch revert -f` |

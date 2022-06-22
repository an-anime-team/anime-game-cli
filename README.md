# 🦀 Anime Game CLI

WIP

## Roadmap to 1.0.0

| status | command | subcommand | description |
| - | - | - | - |
| ✅ | game | info | Get installed game info |
| ❌ | | download | Download the game |
| ❌ | | update | Update the game |
| ✅ | | repair | Repair the game and installed voice packages |
| ✅ | voice | info | List installed voice packages |
| ❌ | | download | Install additional voice package |
| ❌ | | update | Update voice packages |
| ❌ | | remove | Remove voice package |
| ❌ | | repair | Repair (only) voice packages |
| ✅ | patch | info | Get info about linux patch |
| ❌ | | apply | Apply patch |
| ❌ | | update | Update patch |
| ❌ | | revert | Revert patch |
| ✅ | info | | Get info about game, patch and voice packages |
| ❌ | run | | Run the game |

## Mini-wiki

### game repair

| argument | description | example |
| - | - | - |
| `--threads (-t)` | Number of threads used to verify and repair files | `./anime-game-cli game repair -t 12` |
| `--verify-threads (-vt)` | Number of threads used to verify files | `./anime-game-cli game repair -vt 12` |
| `--repair-threads (-rt)` | Number of threads used to repair files | `./anime-game-cli game repair -rt 12` |
| `--ignore (-i, --skip)` | List of names repairer will avoid to repair files with | `./anime-game-cli game repair --ignore='unity,xdelta,report'` |
| `--verify (-v` | Don't repair files and just print broken ones | `./anime-game-cli game repair -v` |

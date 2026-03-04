# FHIST

Tool that provides a simple way to track and manage file history on your local machine.

## Commands

- `fhist help`: Display a short description, available commands and examples. ✅
- `fhist --version`: Show the current version of the tool. ✅

- `fhist add <file_path>`: Add a file to be tracked. ✅
- `fhist remove <id|file_path>`: Stop tracking a file. ✅

- `fhist list`: List all tracked files with their IDs and paths. ✅
- `fhist log <id|file_id>`: Show the history of changes for a specific tracked file. ✅
- `fhist log <id|file_id> --verbose`: Include full snapshot contents in the log output. ✅
- `fhist diff <id|file_id> [--from <snapshot_id>] [--to <snapshot_id>]`: Show a diff between snapshots. ✅
- `fhist rollback <id|file_id> [--snapshot <snapshot_id>]`: Roll back a file to a snapshot. ✅

## Behavior

- A background watcher is started automatically; the first `fhist` process daemonizes and records file changes.
- Tracked files are snapshotted on create/modify events; identical content changes are skipped.
- Paths are resolved to absolute (canonicalized) paths before tracking.
- Data is stored in a local SQLite database under the OS user data directory (e.g. `$XDG_DATA_HOME/fhist/fhist`).

# Hollow Knight CLI Save Converter Tool

Converts encoded Hollow Knight save files to plain text JSON, or encodes plain text JSON into a save file.

Does not overwrite any files, it only creates them to avoid data loss.

```sh
$ hollow-knight-save-converter
Usage: hollow-knight-save-converter --output-file <OUTPUT_FILE> <FILE> <COMMAND>

Commands:
  to-json    Converts encoded save file to plain text json
  from-json  Converts plain text json to encoded save file
  help       Print this message or the help of the given subcommand(s)

Arguments:
  <FILE>  Input file

Options:
  -o, --output-file <OUTPUT_FILE>  Output file
  -h, --help                       Print help
  -V, --version                    Print version
```

## Example usage

```sh
$ cp "/home/deck/.local/share/Steam/steamapps/compatdata/367520/pfx/drive_c/users/steamuser/AppData/LocalLow/Team Cherry/Hollow Knight/user1.dat" "./user1.orig.dat"
$ hollow-knight-save-converter --output-file "./user1.json" "./user1.orig.dat" to-json
$ vd ./user1.json # Edit json with visidata, vim, etc
$ hollow-knight-save-converter --output-file "./user1.modified.dat" "./user1.json" from-json
```

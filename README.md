# 📓 lipu tenpo

lipu-tenpo is a simple reverse time logging tool using [ratatui](https//github.com/tui-rs-revival/ratatui)  
Track your tasks when you move on to something else. No need to remember when you started, it's when the last task ended!

lipu-tenpo is inspired by [Gtimelog](https://github.com/gtimelog/gtimelog)

## Subcommands

lipu-tenpo supports fuzzy subcommand matching (with clap infer_subcommands)

```bash
lipu-tenpo log [DATE]
```

Prints the logs from DATE or today if not specified.

```bash
lipu-tenpo add [ENTRY]
```

Add a new entry using any format supported in Interactive mode

## Interactive Usage
```bash
lipu-tenpo
```

To track time "on task", enter the task you performed with the format `OptionalProject: activity +optional +tags` and lipu-tenpo will append a new log entry. The new entry will have a duration that fills the time since the last entry.

To track time "off task", enter the task but include `**` at the beginning or end.

To start your day it's recommended to enter an "off task" entry.

By default lipu-tenpo tries to read a configuration file from your config directory (using the [directories](https://github.com/dirs-dev/directories-rs) crate)  
If not found, lipu-tenpo writes a default configuration file before running.

```bash
lipu-tenpo -c, --config <CONF_FILE>
```
Use a custom conf file.

```bash
lipu-tenpo -l, --log <LOG_FILE>
```
Use a custom log file


### Optional log entry formatting

You can set the time of a log entry by entering a time (24 hour) before the project/activity:
```
09:00 **arrive
or
17:30 PROJ: debugging chibi robo
```

You can also enter a log with a negative time offset:
```
-15 **arrive (will log an entry 15 minutes ago)
or
-1:30 PROJ: debugging chibi robo (will log an entry 1 hour and 30 minutes ago)
```

## Key Bindings

`Ctrl-Left` / `Ctrl-Right`  
Change active day

`Ctrl-h`  
Jump to today

`Enter`  
Commit log entry

`Ctrl-c` / `Ctrl-q`  
Quit

## Log Format

lipu-tenpo stores your timelog in a csv file (by default in your data directory per the [directories](https://github.com/dirs-dev/directories-rs) crate)   
```
2023-06-14 09:00, , **arrive, 
2023-06-14 09:30, , dev meeting, 
2023-06-14 12:00, GG, making paperclips, +optimization
2023-06-14 13:00, , **lunch, 
2023-06-14 17:15, CB, hunting down a betamax player, 
```
For ease of hand editing the fields aren't quoted so entering a log with a comma will result in a malformed log.
I'm still deciding which way I want to go with this.

## Configuration

tokey uses [TOML](https://toml.io/en/) for configuration

```
virtual_midnight = [0-23] (default = 2)
```

Any entries logged before this hour will belong to the previous day. (e.g. `01:30 PROJ: reticulating splines` would belong to the previous day but `02:00 PROJ: writing treatise on "kepeken e"` wouldn't.)  
This allows you to track your time how you expect rather than have an awkward switch over at midnight.  

## Installation

To do

## License

[MIT](https://mit-license.org/)

# TODO
- [ ] Write install section
- [x] Display current time since last log
- [x] Indicate if on today or not
- [x] Keybind to jump to today
- [ ] Scrollable log list
- [ ] Color preferences
- [ ] Improve tests
- [ ] Improve error handling
- [ ] Translations (toki pona, etc.)
- [ ] Add man page?
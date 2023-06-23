# lipu tenpo

lipu-tenpo is a simple reverse time logging tool using [ratatui](https//github.com/tui-rs-revival/ratatui)  
Track your tasks when you move on to something else. No need to remember when you started, it's when the last task ended!

lipu-tenpo is inspired by [Gtimelog](https://github.com/gtimelog/gtimelog)

## Usage

```bash
lipu-tenpo
```

To track time "on task", enter the task you performed with the format `OptionalProject: activity +optional +tags` and lipu-tenpo will append a new log entry. The new entry will have a duration that fills the time since the last entry.

To track time "off task", enter the task but include `**` at the beginning or end.

To start your day it's recommended to enter an "off task" entry.

By default lipu-tenpo tries to read a configuration file from your data directory (using the [directories](https://github.com/dirs-dev/directories-rs) crate)  
If not found, lipu-tenpo writes a default configuration file before running.

```bash
lipu-tenpo -c, --config <CONF_FILE>
```
Use a custom conf file.

```bash
lipu-tenpo -l, --log <LOG_FILE>
```
Use a custom log file

### Optional log formatting

You can set the time of a log by entering a time (24 hour) before the project/activity:
```
09:00 **arrive
or
17:30 PROJ: debugging chibi robo
```

## Key Bindings

`Ctrl-Left` / `Ctrl-Right`  
Change active day

`Enter`  
Commit log entry

`Ctrl-c` / `Ctrl-q`  
Quit

## Configuration

tokey uses [TOML](https://toml.io/en/) for configuration

```
virtual_midnight = [0-23] (default = 2)
```

Any entries logged before this hour will belong to the previous day. (e.g. `01:30 PROJ: reticulating splines` would belong to the previous day but `02:00 PROJ: churning gender fluid` wouldn't.)  
This allows you to track your time how you expect rather than have an awkward switch over at midnight.  

## Installation

To do

## License

[MIT](https://mit-license.org/)

# TODO
- [ ] Write install section
- [ ] Add offset tracking (-30 logs task 30 minutes ago)
- [ ] Display current time since last log
- [ ] Improve tests
- [ ] Translations (toki pona, etc.)
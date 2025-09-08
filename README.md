# A command line time tracker

"tt" is a time tracker tool, that is designed to not get in your way, but rather do exactly what you need it to do, and nothing more.

```
$ tt status

  Status       Stopped
  Project      app-tt
  Duration     0h 20m
  Start        2025-09-07 20:52
  Stop         2025-09-07 21:12
  Tags         rust
  Description  Convert from perl to rust
  File         $HOME/.TimeTracker/2025/09/20250907-205248_app-tt.trc

```

## Common workflow

Here is a common workflow, where you start tracking time, do some work, and then stop the event at some later point:

```bash
$ tt start --project work 08:00
# Do some work...
$ tt stop 17:00
```

At any time, you can inspect all previous events using the [`report`](#report) sub command or the current event with [`status`](#status). The [`register`](#register) and [`edit`](#edit) commands can be used to add missed events or change existing logged events.

## Installation

You need to install [`cargo`](https://rustup.rs) somehow. After that, you can build and install "tt" with the following command:

```bash
# Install cargo with homebrew
$ brew install rust

# Install (will print location of binary to screen)
$ cargo install --git https://github.com/jhthorsen/app-tt

# Uninstall
$ cargo uninstall tt
```

## Commands

Each command can take `-h`, `--help` for more details, and most switches have a short version, in addition to the `--long` version that is often used for clarity in this document.

### start

This command will start tracking a new event or update an active event. It will also stop the current event if a new event is started. There is also a `--resume` switch which can resume the last logged event.

* The time will default to "now".
* `--project` will default to current working directory.
* `--tag` can add one or more tags to an event.
* `--description` can be used to give the event a longer description.
* `--resume` can be used to resume a previously [stopped](#stop) event.
* `--quiet` will avoid printing the event to screen.

```bash
$ tt start                # Start event
$ tt start --project foo  # Specify project name
$ tt start --tag foo,bar  # Specify tags
$ tt start 09:04          # Specify start time, instead of now
```

### stop

This command will stop tracking the current event.

* The time will default to "now".
* `--quiet` will avoid printing the event to screen.
* `--tag-unless-same-project` will add a tag, unless same project as last event

```bash
$ tt stop                           # Stop event at "now"
$ tt stop 16:00                     # Stop event at 16:00
$ tt stop --tag-unless-same-project # Maybe tag, instead of stopping the event
```

### status

This is the default command and will return the current status.

```bash
$ tt status
```

### report

This command will report how much time you have spent on various events.

* `--project` can be used to filter events by project name.
* `--tag` can be used to find any event with a given tag.
* `--since` will show any event from a given time. Default is the beginning of the current month.
* `--until` will show any event until a given time. Default is "now".
* `--group` will group multiple events in a day into one row in the output.

```bash
$ tt report                     # This month
$ tt report --since 2025-01-01  # Log for this year
$ tt report --tag meetings      # Filter events with tag "meetings"
$ tt report --project work      # Filter events with project name "work"
```

### edit

This command can be used to rewrite log entries with your favorite `$EDITOR`.

* `--since` will edit any event from a given time. Default is the last event's start time.
* `--until` will edit any event from a given time. Default is the last event's start time.
* `--dry-run` will show the commands that would be executed.

```bash
$ tt edit
$ tt edit --since 2025-09-01T09:00:00
$ tt edit --since 2025-09-01T09:00:00 --until 2025-09-01T10:00:00
```

### register

This command is used to register data which has a known start and stop time.

* Start and stop time is required.
* `--project` is required.
* `--tag` can add one or more tags to an event.
* `--description` can be used to give the event a longer description.

```bash
$ tt register 2020-01-01T09:00:00 17:00:00 \
  --project work --description "some description" --tag foo,bar
```

## Environment variables

```bash
EDITOR=vim
TT_MIN_DURATION=300
```

## History

This program used to be written in Perl, but I have lately seen that the [rust](https://www.rust-lang.org/) programming language is not only faster, but also more enjoyable to write.

It is inspired by [App::TimeTracker](https://metacpan.org/pod/App%3A%3ATimeTracker) and share the same log file format, but it has (in my humble opinion) a simpler interface. Either way, this version is way faster.

## Copyright and license

Copyright (C) 2025, Jan Henning Thorsen

This program is free software, you can redistribute it and/or modify it under the terms of the Artistic License version 2.0.

## Author

Jan Henning Thorsen - `jan.henning@thorsenlabs.com`

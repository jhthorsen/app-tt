# NAME

App::tt - Time tracking application

# VERSION

0.18

# DESCRIPTION

[App::tt](https://metacpan.org/pod/App%3A%3Att) is an application that can track how much time you spend on an
project from command line.

It is inspired by [App::TimeTracker](https://metacpan.org/pod/App%3A%3ATimeTracker) and share the same log file format,
but it has (in my humble opinion) a simpler interface and easier to install.

# SYNOPSIS

The application is built up by specifying an command and optional arguments.
Here is a list of example usages, but you can get more details by adding "-h"
after each command.

    # Start tracking time
    $ tt start
    $ tt start -p project-name
    $ tt start -p project-name -t tag1,tag2
    $ tt start -p project-name -t tag1,tag2 09:03

    # Current status
    $ tt
    $ tt status

    # Stop tracking time. Specifiying a time will go back to yesterday,
    # in case you forgot to stop it.
    $ tt stop
    $ tt stop 18:04

    # See the log
    $ tt log
    $ tt log -0year                  # Log for this year
    $ tt log -1year -t meetings      # Log for last year, for tag "meetings"
    $ tt log -p project-name -1month # Log for last month, for project "project-name"
    $ tt log -2                      # Log for two months back
    $ tt export -1                   # Export as CSV

    # Edit the last entry, or a specific file
    $ tt edit
    $ tt edit ~/.TimeTracker/2020/01/20200106-150000_nms.trc

    # Register forgotten time
    $ tt register 2020-01-01T09:00:00 17:00:00 -p "project-name"
    $ tt register 2020-01-01T09:00:00 17:00:00 -p "project-name" -t "tag1,tag2"
    $ tt register 2020-01-01T09:00:00 17:00:00 -p "project-name" -d "description" -t "tag1,tag2"

Basic usage;

    # Start to track time
    $ cd $HOME/git/my-project
    $ tt start

    # Work, work, work, cd ..., do other stuff
    $ tt stop

A more complex example:

    # Start to work on an event and add a tag
    $ tt start -t ISSUE-999 -p some-project-at-work

    # Add another tag to the same event and add a --comment and specify when
    # you stopped working
    $ tt stop -t GITHUB-1005 "Today I was mostly in meetings" 15:24

# CONFIG

Default configuration can be read from either `$PWD/.env` or
`$HOME/.TimeTracker/config`. Here is an example config file with default
settings:

    # some comment
    editor=nano
    export_columns=date,project,hours,rounded,tags,description
    hours_per_month=0  # Used to calculate how much you have to work
    min_time=300       # Will drop the task on "tt stop" if started less than 300 seconds
    round_up_at=30     # Used by the export comnmand to round hours

Can also use the environment aliases:

    EDITOR=vim
    TT_COLUMNS=date,project
    TT_HOURS_PER_MONTH=150
    TT_MIN_TIME=600
    TT_ROUND_UP_AT=15

# ACTIONS

Each command can tak `-h` for more details. Example:

    $ tt start -h

## edit

This command can be used to rewrite a log entry.

    # Edit the last entry with your favorite $EDITOR
    $ tt edit

    # Edit a given file with your favorite $EDITOR
    $ tt edit ~/.TimeTracker/2017/12/20171220-092000_rg.trc

    # Rewrite all the log entries with a perl script
    # See source code before running this command. (Internals might change)
    $ cat rewrite.pl | tt edit

## export

This will export a given set of records as CSV.

    $ tt export         # this month
    $ tt export -2      # two months ago
    $ tt export year    # log for year
    $ tt export -1y     # last year
    $ tt export -p foo  # Filter by project name

## log

This command will report how much time you have spent on various
events.

    $ tt log         # this month
    $ tt log -2      # two months ago
    $ tt log year    # log for year
    $ tt log -1y     # last year
    $ tt log -p foo  # Filter by project name

If you set the "TT\_HOURS\_PER\_MONTH" environment variable to the number of hours
you plan to work per month, then "tt log" will also print how many hours you
have to work in average to meet the target. Example:

    $ TT_HOURS_PER_MONTH=150 tt log -p my_job
    ...
    Remaining this month: 21d,  7:08h/d.

## register

This command is used to import data from other sources. "project-name" default to
"-p" or current git project, "some description" default to "-d" and tags can be
specified by -t foo -t bar

    $ tt register 2020-01-01T09:00:00 17:00:00 -p project-name -d "some description" -t foo -t bar

## start

This command will start tracking a new event. It will also stop the current
event if any event is in process. This command takes the "-p" and "-t"
switches. "-p" (project) is not required if you start from a git repository.

    # Specify a tag and custom project name
    $ tt start -t ISSUE-999 some-project-name

    # Started working at 08:00 instead of now
    $ tt start 08:00

## status

This is the default command and will return the current status:
Are you working on something or not?

    $ tt status

## stop

This command will stop tracking the current event.

    # Stop working at 16:00 instead of now
    $ tt stop 16:00

# COPYRIGHT AND LICENSE

Copyright (C) 2014, Jan Henning Thorsen

This program is free software, you can redistribute it and/or modify it under
the terms of the Artistic License version 2.0.

# AUTHOR

Jan Henning Thorsen - `jhthorsen@cpan.org`

# Tasker

[![Travis Build Status](https://travis-ci.org/chasb96/tasker.svg?branch=master)](https://travis-ci.org/chasb96/tasker)

Tool, similar to Linux' cron jobs, built in Rust, running from JSON configuration files.

Aimed at using JSON files and a filesystem to allow for larger amounts of work and much more configurable options.

Built in Rust to be as light-weight and efficient as possible.

# Examples

## Jobs

Jobs run on an interval: the item specified will run repeatedly, waiting the specified amount of time between each run.

We can note these in our `~/.tasker/config.json` as `jobs` as shown below:

```
{
  "jobs": [
    ... our jobs go here
  ]
}
```

This will list out the jobs to be run, and their details.

Every `job` has three fields: `interval`, the interval to run on in milliseconds, `type`, and the matching value of `type`. An example of this can be found in any `job` example below.

### Available Jobs

As of right now, tasker has `notification`'s, `notification_list`'s, and `system_commands`'s. These will be added as needed or requested.

#### Notifications

A `notification` will throw a notification onto your desktop. A `notification` currently has four fields, shown in the example below:

```
{
  "interval": 1000,
  "type": "notification",
  "notification": {
    "summary": "Here is the title or summary of the notification",
    "body": "Here is the body of the notification",
    "icon": "/path/to/icon",
    "timeout": 1000
  }
}
```
Any or even all of the fields can be left blank or not even specified. Tasker will use a default in it's place.

#### Notification Lists

A `notification_list` is simply a list of notifications to be run. A notification has no fields, it's just an array. 
Example:

```
{
  "interval": 1000,
  "type": "notification_list",
  "notification_list": [
    ... a list of notifications go here
  ]
}
```

#### Shell Commands

A `shell_command` plain and simple just runs commands as if it were in a terminal. It has two fields: `command`, and `args`.
Example:

```
{
  "interval": 1000,
  "type": "shell_command",
  "shell_command": {
    "command": "git",
    "args": [
      "pull",
      "origin",
      "master"
    ]
  }
}
```

`args` can be left blank or not specified. If `args` are not specified, it will just run the command without any args.

## Listeners

Listeners wait for a specific action to happen within your machine.

We can note these in our `~/.tasker/config.json` as `listeners` as shown below:

```
{
  "listeners": [
    ... our listeners go here
  ]
}
```

This will list out the listeners to be run, and their details.

### Available Listeners

Tasker also has `file` listeners. These will be added as needed or requested.

#### Files

A `file` listener does just what it sounds, waits for an event or change on a specific file, and performs an action in response. 

It only has two fields, but is a little more complex than Jobs: it actually takes a job as it's "action in response."

Example:
```
{
  "type": "file",
  "file": {
    "path": "path/to/file",
    "job": {
      ... our job goes here
    }
  }
}
```

## Other

In the future it is planned to include 'Events', jobs that run at a specific time rather than on an interval.


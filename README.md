# Tasker

[![Travis Build Status](https://travis-ci.org/chasb96/tasker.svg?branch=master)](https://travis-ci.org/chasb96/tasker)

Tool to delegate background tasks. Made to create and simplify scheduled commands and event-based commands/event listeners.

Tasker reads from a `JSON` file configured by the user. The `JSON` file is to contain any configuration details and the commands that Tasker is meant to run. Each command will have meta associated with it instructing when to run: this can either be at a specific time, interval, event, or date.

This tool is meant both to be something that actually runs and maintains itself, but also as a learning experience and learning platform, an area to flex and grow my capabilities as a developer.

# Setting Up

There are plans for the future, so currently setup looks wonky to accommodate:
* Desktop notifications

### Overall

A builder file must be specified at `~/tasker.config`, containing the following:

```
{
    "workers": [
        // Workers go here
    ]
}
```

### Workers

"Worker" is just an arbitrary name for anything Tasker does. They're specified as a `JSON` object.

Example:

```
{
    "command": "echo"
}
```

They currently have the following options:
* `"interval": <x>`
    - run the worker on an interval of `x` milliseconds
* `"delay": <x>`
    - wait `x` milliseconds before starting the worker
* `"command": "<executable>"`,
    - the executable to call
    - This comes with an optional array `"args"` specifying the arguments to pass

### Example Config File

```
{
    "configs": [
        "/home/user/second.json"
    ],
    "workers": [
        {
            "delay": 1000,
            "interval": 5000,
            "command": "echo",
            "args": [
                "Hello",
                "World!"
            ]
        }
    ]
}
```

After waiting 1 second, prints `Hello World!` every 5 seconds.

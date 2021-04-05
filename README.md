# `waitfor`

`waitfor` is a command-line utility that executes another command repeatedly,
until it exits with a zero exitstatus.

## Usage

The basic usage is:

    # Wait for a file to be downloaded successfully.
    waitfor curl -O http://example.com/file.tar.gz

    # Wait for a remote host to respond to a ping.
    waitfor ping -c 1 -t 1 192.168.1.42

    # Wait for a FIFO socket file to appear on disk.
    waitfor test -f /tmp/mysql.sock

By default, `waitfor` will run the command with a linear delay of 100ms between
invocations. This can be adjusted with the `--delay` / `-d` options (in
milliseconds).

## Command output

`waitfor` forwards all standard output and error to its standard output and
standard error.

## Backoff and Timeout

`waitfor` can be configured with two types of delays between attempts.

* Linear delay (`--linear` / `-l`, default) will pause a number of milliseconds
  (specified by `--delay` / `-d`) between executions.
* Exponential backoff (`--backoff` / `-b`) will start with the inital delay
  (`--delay` / `-d`) and increase at a provided rate (`--rate` / `-r`).

In all cases, the delay is calculated after an attempt completes, not from the
time an attempt is started, so it cannot guarentee a rate of requests because
that is dependent on the time an individual attempt takes.


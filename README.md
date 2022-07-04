# Simple Discord Schedule Bot

A bot to help schedule basic events.

# Raspberry Pi Setup

1. Download the executables from github, or clone the repository and run `cargo build --release`.
2. Download and install PostgreSQL
```shell
sudo apt install postgresql libpq-dev postgresql-client postgresql-client-common -y
```
3. Create a user

```shell
> sudo su postgres
> createuser pi -P --interactive
```
4. Create a database called `schedule_bot`
```shell
$ psql
> create database schedule_bot;
```
5. Setup Discord bot to run at startup
```shell
> cd /usr/lib/systemd/system
> sudo nano schedule-discord@.service
```
6. In the file enter the below, changing `ExecStart` to the location of the executable, `Environment="DATABASE_URL` to your DB url, `Environment="DISCORD_TOKEN` to your discord token.
You should also add these to `/etc/environment`
```shell
### BEGIN INIT INFO
# Provides:          schedule-discord
# Required-Start:    $all
# Required-Stop:
# Default-Start:     2 3 4 5
# Default-Stop:
# Short-Description: schedule discord server
### END INIT INFO

[Unit]
Description=%I schedule-discord-bot
After=multi-user.target
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=location/of/program %I --no-prompt
User=pi
Group=pi
Type=idle
Restart=always
RestartSec=15
RestartPreventExitStatus=0
TimeoutStopSec=10
Environment="DATABASE_URL=ADD YOUR DB URL HERE"
Environment="DISCORD_TOKEN=ADD YOUR DISCORD BOT TOKEN HERE"

[Install]
WantedBy=multi-user.target
```

7. Enable the service's so they run at startup
```shell
sudo systemctl enable schedule-discord@main
```
8. You will be able to check an output but running
```shell
> sudo journalctl -eu schedule-discord@main
```

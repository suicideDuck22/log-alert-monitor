# log-alert-monitor

## About
---
A very very simple app to run inside WSL to monitor your logs inside a server using SSH.
## WIP
---
This monitor is far from been done. I'm current only learning the language and made this to put some knowladge in practice.

## Premises
---
The log file on the server need to be the creation data on the name. Like: my-app-YYYY-MM-DD.log;
Because the monitor will replace the YYYY-MM-DD with the current date to search inside the most recent log file.
<br>
At this point, the monitor use SSH with Username AND Password to access remote server. This is inserted on .env file.

## Parameters
---
- server_address &rarr; IP Address;
- server_name &rarr; Server identifier to the notification;
- application_name &rarr; Application identifier to the notification;
- logs_path &rarr; Full path since the root directory, including the full name replacing the date information by YYYY-MM-DD. Example: my-app-YYYY-MM-DD.log, will be converted to my-app-2023-03-23.log;

## How it works
Read the folder 'remote-server-infos', where is stored the files containing the information about the server which we are looking the log.
If a file for the current server don't exist, it read all log file and create a file with .info extension containing the date and the quantity of lines.
So on the next call to the server, the monitor know from where him need to start reading, to not read all the log again, and update the file with the new lines count.

This monitor use the [wsl-notify-send](https://github.com/stuartleeks/wsl-notify-send) repository to send notification to the windows from WSL.

## Setting up
---
I will do a script to make this process more easy, but you can start building this project locally using like cargo build.
Move the binary file to a folder of your preference.
Create a folder called remote-servers-infos on this folder, and a .env file containing your credentials.
Now create a notify-send function on your shell.
Like:
```bash
function notify-send(){
    wsl-notify-send.exe --category $WSL_DISTRO_NAME "$1"
}
```

Download the wsl-notify-send [here](https://github.com/stuartleeks/wsl-notify-send/releases/tag/v0.1.871612270)

Unzip and put the .exe on a folder of your user. Add the path to this folder on your PATH environment variable.

Now you will be able to create a cron, who execute the script on WSL passing the parameter mencioneds on this readme, and stay monitoring your logs.

---

Thanks.
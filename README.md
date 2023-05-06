<p align="center">
  <img src="https://github.com/RouHim/rouvens-arch-kickstart/raw/main/icon.png" width="200">
</p>

<p align="center">
    <a href="https://github.com/RouHim/rouvens-arch-kickstart/actions/workflows/pipe.yml"><img src="https://github.com/RouHim/rouvens-arch-kickstart/actions/workflows/ci-cd-pipe.yml/badge.svg" alt="CI"></a>
</p>

<p align="center">
  <a href="https://...">AUR</a>
  | <a href="https://github.com/RouHim/rouvens-arch-kickstart/issues">Issue Tracker</a>
</p>

<p align="center">
This is a collection of scripts and configuration files to install and configure a basic Arch Linux system in my taste.
</p>

## Usage

This application needs to be run as sudo with preserve the HOME environment variable.

```bash
sudo rouvens-arch-kickstart
```

> This is required because the script will use root privileges to install packages and configure the system, but will also
need to access the user's home directory to copy configuration files.

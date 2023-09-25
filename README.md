<p align="center">
  <img src="https://github.com/RouHim/rouvens-arch-kickstart/raw/main/icon.png" width="200">
</p>

<p align="center">
    <a href="https://github.com/RouHim/rouvens-arch-kickstart/actions/workflows/pipe.yml"><img src="https://github.com/RouHim/rouvens-arch-kickstart/actions/workflows/pipe.yaml/badge.svg" alt="CI"></a> |
  <a href="https://github.com/RouHim/rouvens-arch-kickstart/issues">Issue Tracker</a>
</p>

<p align="center">
This is a collection of scripts and configuration files to install and configure a basic Arch Linux system in my taste.
</p>

## Usage

```shell
bash -c "
LATEST_VERSION=$(curl -L -s -H 'Accept: application/json' https://github.com/RouHim/rouvens-arch-kickstart/releases/latest |
  sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/') &&
  curl -L -o rak-x86_64 https://github.com/RouHim/rouvens-arch-kickstart/releases/download/$LATEST_VERSION/rak-x86_64 &&
  chmod +x rak-x86_64
"
```

This application needs to be run without sudo. It will ask for sudo permissions when needed.

```bash
./rak-x86_64
```

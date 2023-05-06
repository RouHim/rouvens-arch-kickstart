#!/usr/bin/env sh

# SUPER + D -> Show desktop
gsettings set org.gnome.desktop.wm.keybindings show-desktop "['<Super>d']"

# SUPER + E -> Open file manager
gsettings set org.gnome.settings-daemon.plugins.media-keys home "['<Super>e']"

# Gnome custom keybinding to open terminator with SUPER + T
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/']"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name "terminator"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command "terminator"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding "<Super>t"


# Change workspace with CTRL + SUPER + ARROW
gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-right "['<Control><Super>Right']"
gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-left "['<Control><Super>Left']"

#!/usr/bin/env sh

# SUPER + D -> Show desktop
echo "Installing: SUPER + D -> Show desktop"
gsettings set org.gnome.desktop.wm.keybindings show-desktop "['<Super>d']"

# SUPER + E -> Open file manager
echo "Installing: SUPER + E -> Open file manager"
gsettings set org.gnome.settings-daemon.plugins.media-keys home "['<Super>e']"

# Gnome custom keybinding to open kitty with SUPER + T
echo "Installing: SUPER + T -> Terminator"
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom9/']"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom9/ name "kitty"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom9/ command "kitty"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom9/ binding "<Super>t"

# Change workspace with CTRL + SUPER + ARROW
echo "Installing: CTRL +SUPER + ARROW -> Switch Workspace"
gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-right "['<Control><Super>Right']"
gsettings set org.gnome.desktop.wm.keybindings switch-to-workspace-left "['<Control><Super>Left']"

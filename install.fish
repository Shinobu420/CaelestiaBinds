#!/usr/bin/env fish

mkdir -p ~/.local/bin
mkdir -p ~/.local/share/applications
mkdir -p ~/.local/share/icons

echo "Building release..."
cargo build --release

if test $status -eq 0
    echo "Installing files..."
    
    cp target/release/caelestia-binds ~/.local/bin/
    
    cp assets/Logo_B.png ~/.local/share/icons/Logo_B.png

    # Use absolute path for Exec in the installed desktop entry to ensure GUI launchers can find it
    string replace "Exec=caelestia-binds" "Exec=$HOME/.local/bin/caelestia-binds" < caelestiabind.desktop > ~/.local/share/applications/caelestiabind.desktop
    
    echo "Updating desktop database..."
    update-desktop-database ~/.local/share/applications/

    echo "Adding path to fish config..."
    fish_add_path ~/.local/bin
    echo "Adding path to bash config..."
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    echo "Adding Keybinds..."
    
    # add window rules and keybinds to hyprland
    set CONF_FILE "$HOME/.config/caelestia/hypr-user.lua"
    set BIND_LINE 'hl.bind("SUPER + Return", hl.dsp.exec_cmd("pkill -x caelestia-binds || ~/.local/bin/caelestia-binds"))'

    mkdir -p (dirname $CONF_FILE)
    touch $CONF_FILE

    if not grep -qF "$BIND_LINE" $CONF_FILE
        echo "Adding keybind to config..."
        echo "$BIND_LINE" >> $CONF_FILE
    else
        echo "Keybind already exists in config. Skipping."
    end
    if not grep -q "caelestiabind-float-center" $CONF_FILE
        echo "Adding windowrule to config..."
        echo '
hl.window_rule({
    name = "caelestiabind-float-center",
    match = { title = "^(CaelestiaBinds)$" },
    float = true,
    center = true
})' >> $CONF_FILE
    else
        echo "Windowrule already exists. Skipping."
    end
    echo "Installation complete!"
else
    echo "Cargo build failed."
end

# Rust Introduction Talk

## TODO

Add explanation of what we're building

```
$ filetype src/main.rs somefile.rb
Rust: src/main.rs
Ruby: src/somefile.rb
``` 

## Wes Notes

### Set Up

#### Displays

Configure screen:

    xrandr --addmode eDP-1 1360x768 # if needed
    xrandr --output eDP-1 --set audio force-dvi --mode 1360x768
    xrandr --output DP-1 --set audio force-dvi --mode 1360x768

Disable notifications.
`simplescreenrecorder` running.

#### Neovim

    call rpcnotify(1, 'Gui', 'Font', 'PragmataPro 24')

#### Tilix

Start terminal in Presentation profile, `cd` to talk directory.

#### Firefox

Open documentation in tab:

    rustup doc

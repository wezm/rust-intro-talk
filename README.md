# Rust Introduction Talk

## Wes Notes

### Set Up

#### Displays

1. Configure screen:

        xrandr --addmode eDP-1 1360x768 # if needed
        xrandr --output eDP-1 --set audio force-dvi --mode 1360x768
        xrandr --output DP-1 --set audio force-dvi --mode 1360x768
        # Ensure xbanish is working

2. Disable notifications.
3. `simplescreenrecorder` running.

#### Neovim

    call rpcnotify(1, 'Gui', 'Font', 'PragmataPro 24')

#### Tilix

1. Start terminal in Presentation profile, `cd` to talk directory.
2. Serve slides in another session: `basic-http-server`.

#### Firefox

1. [Open slides](http://127.0.0.1:4000/slides.html) in new window.
2. Open documentation in tab: `rustup doc`.
3. `f` to fullscreen the slides, Start simplescreenrecorder recording

### Presentation

#### Run with source files

    cargo run -- `fd -t f`

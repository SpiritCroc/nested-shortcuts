# nested-shortcuts

Ever since leaving [AwesomeWM](https://awesomewm.org/) for a Wayland alternative, I've been missing the ["awful" menus](https://awesomewm.org/doc/api/libraries/awful.menu.html), which gave me some kind of multi-layer keyboard shortcuts for launching applications.  
Since I was not able to find any standalone replacement application launcher featuring fully configurable menu hierarchy and keyboard shortcuts, I eventually wrote my own.

## Installation

```
> git clone https://github.com/SpiritCroc/nested-shortcuts.git
> cd nested-shortcuts
> cargo install --path path/where/to/install
```

## Configuration

The menu is defined via yaml, compare [example-menu.yml](example-menu.yml) for an example.

- Every menu entry needs a `title`, which is the rendered name in the menu
- Every menu entry can have an optional `shortcut`, which is a letter on your keyboard that you can press to trigger this menu entry
- Every menu entry needs exactly one of:
    - `exec`: The program to launch
    - `entries`: An array of more menu items to show as submenu

When launching nested-shortcuts, pass it the path to your menu definition as parameter.

## Launching

When piping stdout into a different program, nested-shortcut will just echo the selected executable rather than launching it itself, which can be used to integrate program launches better in your WM.

E.g. for Sway, you could do something like
```
bindsym $mod+z exec /usr/bin/nested-shortcuts path/to/menu.yml | xargs swaymsg exec --
```

## Screenshots

TODO

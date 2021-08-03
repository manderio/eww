# Configuration
Eww has a new configuration format called yuck. Yuck is an S-expression configuration language. Vim users can find a plugin for yuck syntax highlighting [here](https://github.com/elkowar/yuck.vim)

If you are using an older version of eww, and don't want to upgrade to yuck yet, see the [Lecagy](#Legacy) section for the old documentation, though keep in mind this is no longer maintained and more kept as an archive. 

## Placing the configuration file

Note: Example configuration files can be found in the `examples` directory of the repository and are showcased in [Examples](examples.md).

The configuration file and the scss file should lay in `$XDG_CONFIG_HOME/eww` (or, if unset, `$HOME/.config/eww`). The yuck file should be named `eww.yuck` and the scss should be named `eww.scss`
So the directory structure should look like this:
```
$HOME
└──.config
    ──eww
        ├──eww.yuck
        └──eww.scss
```

## Config file

Eww is using the following structure:

```lisp
(defwidget foo []
  (box :class "foo" :orientation "h"
    bar))

(defvar bar
  "This is a variable")

(defwindow bar
  :monitor 0
  (foo))
```

Since yuck is a lisp, you can put your thingys wherever you want, however, keep track of your parenthases. 

For the different thingys that can be included in eww, see the following sections:
[Widgets](#Widgets)
[Windows](#Windows)
[Vars](#Variables)
[Poll](#Poll)
[Listen](#Listen)

## Widgets

Eww is structured around widgets, they are what defines the space and define what goes where within a [window](#windows).

A widget is written in eww like so:

```lisp
(defwidget foo []
  (box
    :class "foo"
    :orientation "h"
    :space-evenly false
    :halign "end"
    "this is a widget too!")

; content ommitted for brevity
```

Widgets can contain any other widgets. In the example above, the built in [box](widgets.md/box) is being used. For more examples of built in widgets, see [Widget Documentation](widgets.md)


## Windows

Windows define where on the screen the widgets should show up. Windows are configured differently depending on whether you are using xorg or wayland.

### Xorg
Example of xorg configuration:

```lisp
(defwindow foo 
  :monitor 0
  :windowtype "dock"
  :geometry (geometry :x "100px" :y "200px" :width "300px" :height "500px")
  :reserve (struts :side "top" :distance "200px")
  :wm-ignore true
  (foo))
```

/* TODO: add table of xorg options */

### Wayland
Example of wayland configuration:

```lisp
(defwindow foo 
  :monitor 0
  :exclusive true
  :focusable false
  :geometry (geometry :x "100px" :y "200px" :width "300px" :height "500px")
  (foo))
```

/* TODO: add table of xorg options */


## Vars


### Poll


### Listen





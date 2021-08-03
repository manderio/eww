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

<<<<<<< HEAD
If you define a variable/widget/window, in a config file, when it's defined somewhere else, you can see a warning in the eww logs (`eww logs`)

## The `<definitions>` block
In here your whole widget will be made, and you can also create your own widgets. Check [Widget Documentation](widgets.md) for pre-defined widgets.

### Custom widgets

Let's get a small config and break it down.

```xml
<definitions>
    <def name="clock">
        <box>
            The time is: {{my_time}} currently.
        </box>
    </def>
    <def name="main">
        <box>
            <clock my_time="{{date}}"/>
        </box>
    </def>
</definitions>

<variables>
    <script-var name="date">
        date
    </script-var>
</variables>
```
That's a long config just for a custom widget. But let's break it down and try to understand it.

This part:
```xml
<def name="clock">
    <box>
        The time is: {{my_time}} currently.
    </box>
</def>
```
Is the custom widget. As we can see by the
```xml
<def name="clock">
```
the widget is called `clock.`Or referenced `<clock>`
The `{{my_time}}` is the value we assign to be well, our time. You can actually set to be anything, it doesn't have to be a time. You can compare it to `value=""`

So if we look at:
```xml
<def name="main">
    <box>
        <clock my_time="{{date}}"/>
    </box>
</def>
```
we can see that we assign `{{my_time}}` to be `{{date}}` and if we look at
```xml
<script-var name="date">
    date
</script-var>
```
we can see that `{{date}}` is simply running the `date` command.

It doesn't have to be `{{my_time}}` either, it can be anything.
```xml
<def name="clock">
    <box>
        The time is: {{very_long_list_of_animals}} currently.
    </box>
</def>
```
is valid.

To use that it would look like this:
```xml
<def name="main">
    <box>
        <clock very_long_list_of_animals="{{date}}"/>
    </box>
</def>
```
## The `<windows>` block

All different windows you might want to use are defined in the `<windows>` block.
The `<windows>` config should look something like this:

```xml
<windows>
    <window name="main_window" stacking="fg" screen="1" windowtype="dock">
        <geometry anchor="top left" x="300px" y="50%" width="25%" height="20px"/>
        <reserve side="left" distance="50px"/>
        <widget>
            <main/>
        </widget>
    </window>
</windows>
```
=======
/* TODO: add table of xorg options */
>>>>>>> 35c8863 (docs update)

### Wayland
Example of wayland configuration:

<<<<<<< HEAD
```xml
    <window name="main_window" stacking="fg" focusable="false" screen="1" exclusive="true">
        <geometry anchor="top left" x="300px" y="50%" width="25%" height="20px"/>
        <widget>
            <main/>
        </widget>
    </window>
```

The window block contains multiple elements to configure the window.
- `<geometry>` is used to specify the position and size of the window.
- `<reserve>` is used to have eww reserve space at a given side of the screen the widget is on.
- `<widget>` will contain the widget that is shown in the window.

There are a couple things you can optionally configure on the window itself:
- `stacking`: stacking describes on what "layer" of the screen the window is shown.
  Possible values on the X11 backend: `foreground "fg"`, `background "bg"`. Default: `"fg"`
  Possible values on the Wayland backend: `foreground "fg"`, `bottom "bt"`, `background "bg"`, `overlay "ov"`. Default: `"fg"`
- `screen`: Specifies on which display to show the window in a multi-monitor setup.
  This can be any number, representing the index of your monitor.
- `exclusive`: Specifies whether or not a surface can be occupied by another.
  A surface can be a window, an Eww widget or any layershell surface.
  The details on how it is actually implemented are left to the compositor.
  This option is only valid on Wayland.
  Possible values: `"true"`, `"false"`. Default: `"false"`
- `focusable`: (Wayland only) whether the window should be able to capture keyboard input.
  Possible values: `"true"`, `"false"`. Default: `"false"`
- `wm-ignore`: (X11 only) wether the window should be managed by the window manager.
  For a centered widget setup this is recommended to be set to true. For a bar, set the windowtype to `dock` instead.
  Note that setting `wm-ignore` will make some other options not work, as those rely on the window manager.
  Possible values: `"true"`, `"false"`. Default: `"true"` except if `<reserve>` is set.
- `windowtype`: (X11 only) Can be used in determining the decoration, stacking position and other behavior of the window.
  Window managers tend to interpret these differently, so play around with which one works for your usecase!
  Possible values:
    - `"normal"`: indicates that this is a normal, top-level window
    - `"dock"`: indicates a bar, dock, or panel window
    - `"utility"`: indicates a pinned utility window
    - `"toolbar"`: toolbars "torn off" from the main application
    - `"dialog"`: indicates that this is a dialog window
    - Default: `"dock"`
- `sticky`: (X11 only) If the window should show up on all workspaces. Note that this may not have any effect, depending on your window manager and the window type.
  Possible values: `"true"`, `"false"`. Default: `"true"`
- `resizable`: (X11 only) If the window should be resizable. Note that this may not have any effect, depending on your window manager and the window type.
  Possible values: `"true"`, `"false"`. Default: `"true"`


### Recommendations for different use-cases on X

Window positioning is... weird on X11. Different window-managers handle things differently, and some things are just not compatible.
Thus, the following setups are recommendations that will _probably_ work. If they don't try to play around with different settings for any of the X11 only properties.

- For a bar:
  - Set `windowtype` to `dock`, and provide a `reserve` configuration to match your window geometry to make the WM reserve space.
  - Set `wm-ignore` to `false`.
- For a centered, full-screen widget setup:
  - Set `wm-ignore` to `true`.
=======
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




>>>>>>> 35c8863 (docs update)

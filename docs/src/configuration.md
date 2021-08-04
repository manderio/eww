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

Since yuck is a lisp, you can define your expressions wherever you want, however, keep track of your parenthases. 

For the different expressions that can be included in eww, see the following sections:
[Widgets](#Widgets)
[Windows](#Windows)
[Vars](#Variables)
[Ewwspressions](#Ewwspressions)

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
If you want ot be able to reuse a value, you can make use of a variable. Eww can make use of four types vars:

- [Var](#Var)
- [Poll](#Poll)
- [Listen](#Listen)
- [Builtin](#Builtin)

Vars take the following arguments:

| Argument      | Explanation           | Applies to | Mandatory |
|:--------------|:----------------------|:-----------|:----------|


You use variables like so in the overall config:

```lisp
(defwidget foo []
  (box :class "foo" :orientation "h"
    bar))

(defwidget bam []
  (box :class "bam" :orientation "v"
    bar))

(defvar bar
  "This is a static variable")
```

### Var

A regular var just contains a static value, and is defied like so:

```lisp
(defvar foo
  "This is a static variable")
```

Notations regarding static variable:
- The value does not update, hence static

/*TODO: Add more notes*/

### Poll

If you want to fetch information in intervals you can use a pol variable, the syntax is as follwing:

```lisp
(defpoll bam
  :interval "2s"
  "bam_output.sh")
```


Notations regarding the poll variable:
- Best used with scripts or short shell commands


### Listen

A listen variable will tail data from a file rather than listening in intervals:

```lisp
(deflisten boom
  "tail -f /seconds/before/boom")
```


Notations regarding listen variable:
- Best for tracking a constant changing value, such as battery information


## Builtin

Builtin variables are already defined and can be used along with other variables to format and fetch data.
An example of making use of a builtin variable is for example to track network usage:

/* TODO: Add example */
```lisp
```

/* TODO: Add the builtin variable list */
Here are the current builtin variables available:

| Name          | Syntax                | Description               |
|:--------------|:----------------------|:--------------------------|


## Ewwspressions
 /* TODO: Add expression language description */


## Summary


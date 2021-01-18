![Crates.io](https://img.shields.io/crates/v/gnome-egpu)
![Crates.io](https://img.shields.io/crates/l/gnome-egpu)

# gnome-egpu

### Tool to switch between iGPU and eGPU on Gnome Wayland


## Installation:

```
$ cargo install gnome-egpu
```


## Instructions:

To set up the initial udev rule:

```
# gnome-egpu setup
```

To then switch:

- Power off computer, plug/unplug eGPU, turn on computer. GDM will attempt to use eGPU, falls back to iGPU.
- **OR:** run ```# gnome-egpu setup``` each time you want to switch, selecting the desired GPU.

**To-Do:**
- ```# gnome-egpu pup``` --- monitors udev changes for eGPU and restarts GDM instantly
- ```# gnome-egpu auto``` --- sets up udev rule to trigger GDM restart automatically

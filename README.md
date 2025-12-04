# !! IMPORTANT !!

## This package is deprecated. Please use an alternative instead!

I didn't have enough time to dedicate to this project unfortunately. 
However, there's much better alternatives with more features available at:

- [hertg/egpu-switcher](https://github.com/hertg/egpu-switcher) (X.Org)
- [ewagner12/all-ways-egpu](https://github.com/ewagner12/all-ways-egpu) (Wayland)

Thanks for your understanding!

---

![Crates.io](https://img.shields.io/crates/v/gnome-egpu)
![Crates.io](https://img.shields.io/crates/l/gnome-egpu)

# ~~gnome-egpu~~

### ~~Tool to switch between iGPU and eGPU on Gnome Wayland~~

## ~~Dependencies:~~

- ~~```libgudev-1.0```~~
- ~~```libglib2.0```~~

## ~~Installation:~~

~~```$ cargo install gnome-egpu```~~


## ~~Instructions:~~

~~To set up the initial udev rule:~~

~~```# gnome-egpu setup```~~

~~To then switch:~~

- ~~Power off computer, plug/unplug eGPU, turn on computer. GDM will attempt to use eGPU, falls back to iGPU.~~
- ~~**OR:** run ```# gnome-egpu setup``` each time you want to switch, selecting the desired GPU.~~

~~**To-Do:**~~
- ~~```# gnome-egpu pup``` --- monitors udev changes for eGPU and restarts GDM instantly~~
- ~~```# gnome-egpu auto``` --- sets up udev rule to trigger GDM restart automatically~~

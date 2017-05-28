# GVim Remote
Open file in existing gvim instance, same as `alias vr='gvim --remote-tab-silent'` in \*unix shell.  
It is for Windows GVim, with a vim icon.

# Build 

```
cargo build --release
```

Note that `winres` cannot detect Windows SDK correctly, I make my own [fork](https://github.com/bennyyip/winres) and it only works for SDK 10586.  
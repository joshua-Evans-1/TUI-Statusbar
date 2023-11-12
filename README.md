# tui statusbar

## install 

```cargo install cargo-generate```

```cargo generate --git https://github.com/joshua-Evans-1/TUI-Statusbar```


## bspwm.rc 

most important lines are

```bspc rule -a kitty:statusbar sticky=on state=floating rectangle=3790x70+20+20```

```kitty --name statusbar sh -c 'cd <DIR_TO_PROJECT> && cargo run'```


```
#! /bin/sh

pgrep -x sxhkd > /dev/null || sxhkd &

bspc monitor -d I II III IV V VI VII VIII IX X

bspc config border_width        5
bspc config window_gap          20
bspc config top_padding         100 

bspc config split_ratio          0.52
bspc config borderless_monocle   true
bspc config gapless_monocle      true

bspc rule -a Gimp desktop='^8' state=floating follow=on
bspc rule -a Chromium desktop='^2'
bspc rule -a mplayer2 state=floating
bspc rule -a Kupfer.py focus=on
bspc rule -a Screenkey manage=off

bspc rule -a kitty:statusbar sticky=on state=floating rectangle=3790x70+20+20

kitty --name statusbar sh -c 'cd ~/projects/statusbar && cargo run' 
```


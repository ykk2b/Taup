# Terminal-based Audio Player (TAUP)


```sh
taup play [file|directory] # play the audio file / directory
-s, --sort [title|artist|album|year|genre|track|comment|duration]
## adds hotkeys
[R]|"no repeate" | "repeate"
[M]| "mute" | "unmute"
[H]| "no shuffle" | "shuffle" # disabled in file mode
[E]| "play next" # disabled in file mode
[Q]| "play previous" # disabled in file mode
[J / _]| "play" | "pause"
[arrow_up / W]| "volume up"
[arrow_down / S]| "volume down"
[arrow_left / D]| "forward 5s"
[arrow_right / A]| "forward back 5s"


taup edit [file] # edit the metadata of the file
-t, --title [title]
-a, --artist [artist]
-l, --album [album]
-y, --year [year]
-g, --genre [geenre]
-t, --track [track]
-c, --comment [comment]


taup view [file] # display the metadata of the file


taup download [link] [dir] (name) ## download {link} in the {dir} directory with {name} name
-o, --option [opt1|opt2] # mirrors

taup browse [dir] # browse directory musics (just list)
```

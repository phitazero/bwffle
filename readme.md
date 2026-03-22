BWFFLE (pronounced baffle) - Black&White FastFetch Logo Editor

Wrote this thing to tweak one fastfetch logo (the YoRHa logo processed with `chafa` with `--symbols block`, looks real cool) and probably never use it again.

In my terminal #000000 is considered transparent, so when when chafa emits this bg color holes appear. Hence you can set the bg of a symbol to "black" (#000001). Looks black and isn't transparent.

Also, unlike the versatile and potentially small block symbols, background color is fixed size, so when chafa emits an almost-black background for a small block symbol in my transparent terminal it looks like an ugly black chuck sticking out. Hence you can set the bg of a symbol to real black #000000, which is actually transparent.

The logo also has one symbol I wanna remove, so I made the erase symbol option.

WASD/arrow keys - navigation  
b - set to black  
t - set to transparent  
e - erase character  
z/u - undo  
q - render the modified logo into stdout (intermediate renders are into stderr) and exit  

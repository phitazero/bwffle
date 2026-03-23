BWFFLE (pronounced baffle) - Black&White FastFetch Logo Editor

Wrote this thing to tweak one fastfetch logo (the YoRHa logo processed with `chafa` with `--symbols block`, looks real cool) and probably never use it again.

In my terminal #000000 is considered transparent, so when when chafa emits this bg color holes appear. Hence you can set the bg of a symbol to "black" (#000001). Looks black and isn't transparent.

Also, unlike the versatile and potentially small block symbols, background color is fixed size, so when chafa emits an almost-black background for a small block symbol in my transparent terminal it looks like an ugly black chuck sticking out. Hence you can set the bg of a symbol to real black #000000, which is actually transparent.

In certain cases, for certain reasons, instead of rendering white symbols on black background chafa emits black symbols on white background, and the symbols can't be set to transparent. The invert function swaps the FG and BG colors, and changes the block symbols to its inverse version (e. g. lower 3/4 - upper 1/4). Some symbols are from a legacy set, and chafa doesn't use them (probably for a reason), so creating the problem. I, a gigachad, am not afraid to use them.

The logo also has one symbol I wanna remove, so I made the erase symbol option.

WASD/arrow keys - navigation  
b - set to black  
t - set to transparent  
e - erase character  
i - invert  
z/u - undo  
q - render the modified logo into stdout (intermediate renders are into stderr) and exit  

# Color themes

A color themes defines several colors with specific names that can then be used
in the body of the beamer documented created.

The most important colors are
* `thm_bg` color of the background,
* `thm_blc` color used to balance the background, and
* `thm_fg` color of the foreground.

`thm_blc` is used to balance the custom colors. Typically the balance color is
the same as the foreground color, but sometimes it's not. In the *white on
black* theme for instance,
* `thm_bg` is black,
* `thm_blc` is white, and
* `thm_fg` is white.
But in the *cyan on black* theme,
* `thm_bg` is black,
* `thm_blc` is white, and
* `thm_fg` is cyan.
That is, white will be used to balance colors, not cyan which would mess up all
the coloring.
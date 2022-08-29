# DRONE

Developed from the base of Hands-on rust, a book i recommend for anyone getting into rust with an already established programming background.

Through the book I got to develop my own dungeon crawler rogulike game! This is the result. Some extra implementations i did after finishing the book were
- Adventure Log
- New enemy variety 
- New items
- Decals on map for decoration
- Added more UI elements
- Adjusted rendering layers
- Webassembly build (!!!)
- New graphics (made by Emil Bertholdsson)

[You can play the game on itch.io](https://davidstyrbjorn.itch.io/drone) (It loads and runs very fast directly in the browser 😊)

![Imgur](https://i.imgur.com/VApinXC.png)

## Libraries
The game in its core uses *Legion* to get ECS into the code. *bracket-lib* to render glyphs and such to a window. *wasm-pack* for easy compilation the webassembly target. 

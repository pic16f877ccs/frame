## frame

Display text in a frame.

## Description
Displays text within predefined or custom frames. 
You can set the frame color, change the text alignment.
The frame can be centered and also resized.

### Installation

Build and install with Rust package manager.
```console
cargo install frame
```
Installing the [package](https://github.com/pic16f877ccs/frame/releases/download/v0.3.4/frame_0.3.4_amd64.deb) using the Ubuntu package manager.
```console
sudo apt install ./frame_0.3.4_amd64.deb
```

### Usage:

```console
frame -f'double' -c'cyan' ./file
```
#### Double frame option

<img src="img/frame_double.png" width=60% height=60%>

```console
frame -a'centr' -c'magenta' ./file
```
#### Justify text centered

<img src="img/frame_centr.png" width=60% height=60%>

## License
GNU General Public License v3.0

# todo-updater
Rust program to update my conky todo list

## Installation
Clone this repo:
```bash
git clone https://github.com/Dartt0n/todo-updater.git
```
Go to directory and build it
```bash
cd todo-updater
cargo build --release
```
Executable file will be ./target/release/todo-updater

## Misc

I use it for update my conky todo.

For example you can add
```
conky.text = [[...
$hr
TODO:
${font Hack\ Nerd\ Font\ Mono}${exec cat /home/dartt0n/.config/conky/todo.txt}$font
...
]]
```
to your conky config

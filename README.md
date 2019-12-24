# `sloth` - your handy productivity buddy

Procrastinating when there are tasks you need to get done? Sloth will tell you what to do:

```
_____
____ \_mo__mo__mo_   ___             __
    \_____________lll__ \_____   ___/ _
                /$\,.\ \______lll____/
               |$$$\,.\   /$$/|.,\
      _________|$$| |,.\ /$$/ |,.,l
     y"""    """\$| |.,.|$$$| |.,.|
    |"()"    "()"|$\|,.,|$$$$V.,.,|
    |"""  oo  """|$/,.,.|$$$/.,.,.|
    |"   >=     "|y,.,.,.\$/.,.,.,|
     '.________.;.,.,.,.,.,.,.,.,.|
        \,.,.,.,.,.,.,.,.,.,.,.,.,|
     /   \,.,.,.,.,.,.,.,.,.,.,.,.|
    /     '..,.,.,.,.,.,.,.,.,.,./
   /        '.mmmmmmmmmmmmmmmmmm!
 +--------------------+
 | Christmas shopping |
 +--------------------+
```

`sloth` picks a random task from a `sloth_tasks.txt` file and tells you to go do it.

# Installation

This program is written in rust and uses `cargo`, rust's package manager.
There is a `Makefile` that interally just invokes cargo to build the binary.
To install, clone the repo and install rust.
You can try building by running `make` (or `cargo build`).
Do `make install` to install the binary to `/usr/local/bin`.
You can install to some other location by setting the `PREFIX` variable.
For example, `PREFIX=FOO make install` will install the binary to `FOO/bin`.

# Usage

To use sloth, first create your sloth `tasks.txt` file.
The default location is `$HOME/.sloth/tasks.txt`, but this can be overriden by setting the `SLOTH_TASKS` variable.
Populate your `tasks.txt` file with tasks you need to do.
Tasks should be separated by an empty line.
When you run the `sloth` binary, it will pick one of these tasks at random and tell you to go do it.

# Etc

This project was inspired by other silly ASCII programs like `cowsay` and `sl`.
This is my first time writing a rust program, so I probably did something in a dumb way.
The sloth ASCII art is by me.
Feel free to use it for whatever.

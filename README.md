Experimenting with Rust and systems programming.

# Setup

Install rust, and build & run the OS itself:

```bash
$ sudo apt-get install QEMU xorriso nasm                    # Debian/Ubuntu
$ sudo pacman -S xorriso nasm mtools && yaourt -S qemu-git  # Arch (problems with pacman -S qemu?)
$ make rust
$ make run
```

## For GDB debugging

Need to build the [patched GDB](http://os.phil-opp.com/set-up-gdb.html) so that
we can attach from the beginning of the start process. Build, install, and run
with GDB attached by doing:

```bash
$ sudo apt-get install texinfo flex bison python-dev ncurses-dev
$ make gdb
```

# References

## Starting point
- http://os.phil-opp.com/multiboot-kernel.html (working through this to start)
- http://wiki.osdev.org/
### Paging
- http://stackoverflow.com/questions/18431261/how-does-x86-paging-work

## Keyboards and stuff
- http://jvns.ca/blog/2013/12/04/day-37-how-a-keyboard-works/
- http://www.randomhacks.net/bare-metal-rust/

## TCP!
- http://www.saminiir.com/lets-code-tcp-ip-stack-1-ethernet-arp/

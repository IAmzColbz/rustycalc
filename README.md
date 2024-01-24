This is a quick program I rewrote in Rust from my original python calculator. This version is much cleaner and more structured.
The only operations in this version, much like my python calculator, are addition, subtraction, multiplication, and division.

*To run on Windows:*
Download the executable and run it via cmd or powershell? Frankly idk how windows does their executable permissions and this is a commandline based calculator.
I don't know if it works on windows, but I did add some windows specific runtime commands just in case it does.

*To run on Linux:*
Download the application/executable file to a directory of your choice. Open that directory via CLI and type the following commands:
```shell
chmod +x ./rustycalc && ./rustycalc
```

*for mac users:*
¯\_(ツ)_/¯

This program uses 64 bit floating point precision meaning **sure** you *could* round to a thousand digits, but its going to stop computing the floating point after only a few digits, and even before that its accuracy drops drastically. keep that in mind.

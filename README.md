This is a quick program I rewrote in Rust from my original python calculator. This version is much cleaner and more structured.
The only operations in this version, much like my python calculator, are addition, subtraction, multiplication, and division.

*To run on Windows:*
There are two releases, a raw binary for linux and an exe for windows. Simply download the .exe version and open it as an application, it will trip windows defender because it is an unsigned exe file. To bypass the prompt, just hit the extra context menu in the popup and hit 'run anyway.' All the source code is in the repo if you want to verify that it's not malicious.
Tested on Windows 11 devkit VM

*To run on Linux:*
Download the application/executable file to a directory of your choice. Open that directory via CLI and type the following commands:
```shell
chmod +x ./rustycalc && ./rustycalc
```

*for mac users:*
¯\_(ツ)_/¯ I genuinely know nothing about the functionality if Mac. I ran windows in a VM to compile it for the release but other than that I don't know if Mac can use the linux binary or if it has the ability to use the .exe or if I have to find a way to compile it specifically for the MacOS. Beats me.

This program uses 64 bit floating point precision meaning **sure** you *could* round to a thousand digits, but its going to stop computing the floating point after only a few digits, and even before that its accuracy drops drastically. keep that in mind.

# DanGanRonpa Tools
This is a program designed to allow easy script editing for the video game Danganronpa, by quickly decompiling and recompiling the script files.

## Installation
A python virtual enviroment is recommended.
Pip install Maturin and Tkinter for python dependencies.
Use 'maturin develop' inside the 'dgrlin' directory, or use the given compile.bat file.
Run main.py

## Usage
You can decompile a .bin or .bytecode example file inside the /data directory.
(This is a good portion of what the .lin is made of, but I am not there yet)

You can recompile the decompiled .txt file back into a .bin.

## Future Plans
Immediate: 
* Remove all the .unwraps I threw hap-hazardly into here.
* Invalid opcode parsing should bubble up into python to be displayed in the GUI.

After that, I will work on complete lin parsing.

# DanGanRonpa Tools
This is a program designed to allow for easy script editing for the video game Danganronpa, by quickly decompiling and recompiling the '.lin' script files.

## Installation
A python virtual enviroment is recommended.
Pip install Maturin and Tkinter for python dependencies.
Use the given compile.bat file to generate the rust library for python and load the gui.
Run main.py for future uses.

## Usage
Click the decompile button and selecte the file[s] you would like to decompile. Select the output folder for the resulting .txt's.
Recompile with the compile button and select the .txt file[s] you would like to recompile. Select the output folder for your new .lin's.

## Future Plans
* Detect any poorly formed lines (Be careful right now)
* Macros as shorthand for common line combonations
* Enums for chatacter names, to replace the default integer id.

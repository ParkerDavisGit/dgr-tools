import dgrlin

import logging
import sys

import rust_interfacer

from tkinter import *
from tkinter import ttk
import tkinter.filedialog

class Window:
    def __init__(self):
        ### Variables
        self.last_compile_input  = "./data"
        self.last_compile_output = "./output"

        self.last_decompile_input  = "./data"
        self.last_decompile_output = "./output"

        ### Set Up Window
        self.set_up_logging()

        ### Main Window
        self.root = Tk()
        self.root.resizable(width=False, height=False)
        self.root.geometry("800x500")
        self.root.title("DGR TOOLS")
        self.logger.info("window created")

        bg = PhotoImage(file = "assets/background.png") 
        self.background_label = ttk.Label(self.root, image=bg)
        self.background_label.place(x = 0, y = 0)

        #main_frame = ttk.Frame(self.root, width=800, height=500)
        #main_frame.place(x=0, y=0)

        #self.header_label = ttk.Label(self.root, text="Gello Gorld!", width=100).place(x = 350, y = 25)
        self.compile_button = ttk.Button(self.root, text="Compile", command=lambda: rust_interfacer.compile_lin()).place(x = 25, y = 75)
        self.decompile_button = ttk.Button(self.root, text="Decompile", command=lambda: rust_interfacer.decompile_lin()).place(x = 25, y = 150)

        self.log_frame = ttk.Frame(self.root, width=400, height=200)
        self.log_frame.place(x=200, y=200)

        self.file_log = Label(self.log_frame, text="", width=50)
        self.file_log.place(x=0, y=0)
        self.logger.info("widgets created")

        self.root.mainloop()

    def set_up_logging(self, level: int = logging.DEBUG) -> None:
        self.logger = logging.getLogger(__name__)
        self.logger.setLevel(level)

        handler = logging.StreamHandler(sys.stderr)

        handler.setFormatter(   
            logging.Formatter("%(name)s".center(12)+" - %(levelname)s - %(message)s")
        )

        self.logger.addHandler(handler)

    def update_log(new_line):
        ...
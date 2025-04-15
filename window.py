import dgrlin

import logging
import sys

from tkinter import *
from tkinter import ttk
import tkinter.filedialog

class Window:
    def __init__(self):
        self.set_up_logging()

        self.root = Tk()
        self.root.resizable(width=False, height=False)
        self.root.geometry("800x500")
        self.root.title("DGR LIN PARSER")
        self.logger.info("window created")

        main_frame = ttk.Frame(self.root, padding = 10)
        main_frame.grid()

        self.header_label = ttk.Label(main_frame, text="Gello Gorld!").grid(column=0, row=0)
        self.compile_button = ttk.Button(main_frame, text="Compile", command=lambda: self.open_file("compile")).grid(column=0, row=1)
        self.decompile_button = ttk.Button(main_frame, text="Decompile", command=lambda: self.open_file("decompile")).grid(column=1, row=1)

        self.file_open_log = Label(main_frame, text="")
        self.file_open_log.grid(column=0, row=2)
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

    def open_file(self, mode):
        if mode == "compile":
            self.logger.info("opening file dialog")
            filename = tkinter.filedialog.askopenfilename(initialdir = "./data",
                                            title = "Select a text file!",
                                            filetypes = (("Text files", "*.txt*"),
                                                        ("Text files", "*.txt*")))
            
            if filename == "":
                self.logger.warning("user did not select file")
                self.file_open_log.configure(text="No file selected!")
                return

            dgrlin.compile(filename)

        elif mode == "decompile":
            self.logger.info("opening file dialog")
            filename = tkinter.filedialog.askopenfilename(initialdir = "./data",
                                            title = "Select a binary!",
                                            filetypes = (("Linary files", "*.lin*"),
                                                        ("Binary files", "*.bin*")))   
            
            if filename == "":
                self.logger.warning("user did not select file")
                self.file_open_log.configure(text="No file selected!")
                return
            
            dgrlin.decompile(filename)
        
        self.file_open_log.configure(text=f"File {mode}d: {filename}")


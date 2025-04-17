import dgrlin

import logging
import sys

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

        self.root = Tk()
        self.root.resizable(width=False, height=False)
        self.root.geometry("800x500")
        self.root.title("DGR TOOLS")
        self.logger.info("window created")

        main_frame = ttk.Frame(self.root, padding = 10)
        main_frame.grid()

        self.header_label = ttk.Label(main_frame, text="Gello Gorld!").grid(column=0, row=0)
        self.compile_button = ttk.Button(main_frame, text="Compile", command=lambda: self.compile_lin()).grid(column=0, row=1)
        self.decompile_button = ttk.Button(main_frame, text="Decompile", command=lambda: self.decompile_lin()).grid(column=1, row=1)

        self.file_open_log = Label(main_frame, text="")
        self.file_open_log.grid(column=1, row=2)
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


    def compile_lin(self):
        ## CHOOSE INPUT
        self.logger.info("opening file dialog")
        input_filenames = tkinter.filedialog.askopenfilenames(initialdir = self.last_compile_input,
                                        title = "Select a text file!",
                                        filetypes = (("Text files", "*.txt*"),
                                                    ("Text files", "*.txt*")))

        if input_filenames == "":
            self.logger.warning("user did not select file")
            self.file_open_log.configure(text="No file selected!")
            return

        self.last_compile_input = input_filenames[0].rsplit("/", 1)[0]

        ## CHOOSE OUTPUT
        output_folder = tkinter.filedialog.askdirectory(initialdir = self.last_compile_output,
                                        title = "Select an output folder!")
        
        if output_folder == "":
            self.logger.warning("user did not select output folder")
            self.file_open_log.configure(text="No folder selected!")
            return
        
        self.last_compile_output = output_folder.rsplit("/", 1)[0]

        for input_filename in input_filenames:
            try:
                dgrlin.compile(input_filename, output_folder)
            except RuntimeError as err:
                print(err)
            except:
                print("Unknown Error")
        
        self.file_open_log.configure(text=f"File compiled: {input_filename}")
    

    def decompile_lin(self):
        ## CHOOSE INPUT
        self.logger.info("opening file dialog")
        input_filenames = tkinter.filedialog.askopenfilenames(initialdir = self.last_decompile_input,
                                        title = "Select a binary!",
                                        filetypes = (("Linary files", "*.lin*"),
                                                    ("Binary files", "*.bin*")))   
        
        ## For some unkown reason, 'askopenfilenames' returns a tuple when any file[s] is selected.
        ## But if no files are selected, it returns an empty string.
        if input_filenames == "":
                self.logger.warning("user did not select file")
                self.file_open_log.configure(text="No file selected!")
                return

        self.last_decompile_input = input_filenames[0].rsplit("/", 1)[0]

        ## CHOOSE OUTPUT
        output_folder = tkinter.filedialog.askdirectory(initialdir = self.last_decompile_output,
                                        title = "Select an output folder!")
        
        self.last_decompile_output = output_folder.rsplit("/", 1)[0]
        
        if output_folder == "":
            self.logger.warning("user did not select output folder")
            self.file_open_log.configure(text="No folder selected!")
            return

        for input_filename in input_filenames:
            try:
                dgrlin.decompile(input_filename, output_folder)
            except RuntimeError as err:
                print(err)
                return
            except:
                print("Unknown Error")
        
        self.file_open_log.configure(text=f"File decompiled: {input_filename}")


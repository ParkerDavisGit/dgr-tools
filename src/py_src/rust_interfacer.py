import dgrlin

import logging
import sys

import tkinter.filedialog

def compile_lin(self):
    ## CHOOSE INPUT
    self.logger.info("opening file dialog")
    input_filenames = tkinter.filedialog.askopenfilenames(initialdir = self.last_compile_input,
                                    title = "Select a text file!",
                                    filetypes = (("Text files", "*.txt*"),
                                                ("Text files", "*.txt*")))

    if input_filenames == "":
        self.logger.warning("user did not select file")
        self.file_log.configure(text="No file selected!")
        return

    self.last_compile_input = input_filenames[0].rsplit("/", 1)[0]

    ## CHOOSE OUTPUT
    output_folder = tkinter.filedialog.askdirectory(initialdir = self.last_compile_output,
                                    title = "Select an output folder!")
    
    if output_folder == "":
        self.logger.warning("user did not select output folder")
        self.file_log.configure(text="No folder selected!")
        return
    
    self.last_compile_output = output_folder.rsplit("/", 1)[0]

    for input_filename in input_filenames:
        try:
            dgrlin.compile(input_filename, output_folder)
        except RuntimeError as err:
            self.file_log.configure(text=err.__str__().split("\n")[0])
        except:
            print("Unknown Error")
    
    self.file_log.configure(text=f"File compiled: {input_filename}")
    

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
            self.file_log.configure(text="No file selected!")
            return

    self.last_decompile_input = input_filenames[0].rsplit("/", 1)[0]

    ## CHOOSE OUTPUT
    output_folder = tkinter.filedialog.askdirectory(initialdir = self.last_decompile_output,
                                    title = "Select an output folder!")
    
    self.last_decompile_output = output_folder.rsplit("/", 1)[0]
    
    if output_folder == "":
        self.logger.warning("user did not select output folder")
        self.file_log.configure(text="No folder selected!")
        return

    for input_filename in input_filenames:
        try:
            dgrlin.decompile(input_filename, output_folder)
        except RuntimeError as err:
            self.file_log.configure(text=err.__str__().split("\n")[0])
            return
        except:
            print("Unknown Error")
    
    self.file_log.configure(text=f"File decompiled: {input_filename}")
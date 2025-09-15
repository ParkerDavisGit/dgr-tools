import dgrlin

import logging
import sys
import os

import rust_interfacer

import tkinter as tk
import tkinter.ttk as ttk
import tkinter.filedialog

import configparser

class Window:
    def __init__(self):
        ### Configuration Setup
        self.config = configparser.ConfigParser()
        self.config.read("config.ini")
        print()

        ### Variables
        self.last_compile_input  = "./data"
        self.last_compile_output = "./output"

        self.last_decompile_input  = "./data"
        self.last_decompile_output = "./output"

        ### Set Up Window
        self.set_up_logging()

        ### Main Window
        self.root = tk.Tk()
        self.root.resizable(width=False, height=False)
        self.root.geometry(f"{self.ConfigSectionMap("WindowSettings")['windowwidth']}x{self.ConfigSectionMap("WindowSettings")['windowheight']}")
        self.root.title("DGR TOOLS")
        self.logger.info("window created")

        bg = tk.PhotoImage(file = "assets/background.png") 
        self.background_label = ttk.Label(self.root, image=bg)
        self.background_label.place(x = 0, y = 0)

        #main_frame = ttk.Frame(self.root, width=800, height=500)
        #main_frame.place(x=0, y=0)

        #self.header_label = ttk.Label(self.root, text="Gello Gorld!", width=100).place(x = 350, y = 25)
        self.compile_button   = tk.Button(self.root, text="Compile",   width=18, height=3, command=lambda:  self.compile_lin()).grid(row=0, column=0, padx=(50, 0), pady=(50, 0))
        self.decompile_button = tk.Button(self.root, text="Decompile", width=18, height=3, command=lambda: self.decompile_lin()).grid(row=0, column=1, padx=(25, 0), pady=(50, 0))

        self.log_frame = ttk.Frame(self.root, width=300, height=200)
        self.log_frame.grid(row=1, column=0, padx=(50, 0), pady=(50, 0), columnspan=2)

        self.file_log = tk.Label(self.log_frame, text="", justify="left")
        self.file_log.place(x=0, y=0)
        self.logger.info("widgets created")

        self.tree = ttk.Treeview(self.root, height=35)
        self.tree.grid(row=0, column=2, padx=(150, 0), pady=(50, 0), rowspan=20)
        self.tree.column('#0', width=800)

        # Inserted at the root, program chooses id:
        # self.tree.insert('', 'end', 'widgets', text='Widget Tour')
        
        # # Same thing, but inserted as first child:
        # tree.insert('', 0, 'gallery', text='Applications')

        # # Treeview chooses the id:
        # id = tree.insert('', 'end', text='Tutorial')

        # # Inserted underneath an existing node:
        # tree.insert('widgets', 'end', text='Canvas')
        # tree.insert(id, 'end', text='Tree')

        # tree.insert('', 'end', text='button', tags=('ttk', 'simple'))
        # tree.tag_configure('ttk', background='yellow')
        self.tree.bind('<Double-1>', self.itemClicked)

        ###
        self.log_frame_lines = []

        ###

        self.root.mainloop()
    
    def itemClicked(self, x):
        print(x.widget.focus())
        self.beginLoadOfFileTree()
        

    def compile_lin(self):
        ## CHOOSE INPUT
        self.logger.info("opening file dialog")
        input_filenames = tkinter.filedialog.askopenfilenames(initialdir = self.last_compile_input,
                                        title = "Select a text file!",
                                        filetypes = (("Text files", "*.txt*"),
                                                    ("Text files", "*.txt*")))

        if input_filenames == "":
            self.logger.warning("user did not select file")
            self.update_log("No file selected!")
            return

        self.last_compile_input = input_filenames[0].rsplit("/", 1)[0]

        ## CHOOSE OUTPUT
        output_folder = tkinter.filedialog.askdirectory(initialdir = self.last_compile_output,
                                        title = "Select an output folder!")
        
        if output_folder == "":
            self.logger.warning("user did not select output folder")
            self.update_log("No folder selected!")
            return
        
        self.last_compile_output = output_folder.rsplit("/", 1)[1]

        for input_filename in input_filenames:
            try:
                dgrlin.compile(input_filename, output_folder)
                self.update_log(f"File compiled: {input_filename}")
            except RuntimeError as err:
                self.update_log(err.__str__().split("\n")[0])
            except:
                print("Unknown Error")
        
        

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
                self.update_log("No file selected!")
                return

        self.last_decompile_input = input_filenames[0].rsplit("/", 1)[0]

        ## CHOOSE OUTPUT
        output_folder = tkinter.filedialog.askdirectory(initialdir = self.last_decompile_output,
                                        title = "Select an output folder!")
        
        self.last_decompile_output = output_folder.rsplit("/", 1)[1]
        
        if output_folder == "":
            self.logger.warning("user did not select output folder")
            self.update_log("No folder selected!")
            return

        for input_filename in input_filenames:
            try:
                dgrlin.decompile(input_filename, output_folder)
                self.update_log(f"File decompiled: {input_filename}")
            except RuntimeError as err:
                self.update_log(err.__str__().split("\n")[0])
                return
            except:
                print("Unknown Error")


    def set_up_logging(self, level: int = logging.DEBUG) -> None:
        self.logger = logging.getLogger(__name__)
        self.logger.setLevel(level)

        handler = logging.StreamHandler(sys.stderr)

        handler.setFormatter(   
            logging.Formatter("%(name)s".center(12)+" - %(levelname)s - %(message)s")
        )

        self.logger.addHandler(handler)

    def update_log(self, new_line):
        if self.log_frame_lines.__len__() > 11:
            self.log_frame_lines.pop(11)
        
        self.log_frame_lines.insert(0, new_line)

        self.file_log.configure(text="\n".join(self.log_frame_lines))
    
    ## ConfigParser helper function taken from the python website.
    def ConfigSectionMap(self, section):
        dict1 = {}
        options = self.config.options(section)
        for option in options:
            try:
                dict1[option] = self.config.get(section, option)
                if dict1[option] == -1:
                    print("skip: %s" % option)
            except:
                print("exception on %s!" % option)
                dict1[option] = None
        return dict1


    def beginLoadOfFileTree(self):
        starting_folder = tkinter.filedialog.askdirectory(initialdir = ".",
                                        title = "Select a folder to load!")
        
        if starting_folder == "":
            self.logger.warning("user did not select a folder to load")
            self.update_log("No folder selected!")
            return
        
        self.loadFileTreeRecursion(starting_folder, '')

    
    def loadFileTreeRecursion(self, folder_path, parent_id):
        files = os.listdir(folder_path)

        for file in files:
            self.tree.insert(parent_id, 'end', parent_id + '/' + file, text = file)
            if "." not in file:
                self.loadFileTreeRecursion(folder_path + '/' + file, parent_id + '/' + file)
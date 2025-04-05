import dgrlin
from tkinter import *
from tkinter import ttk
import tkinter.filedialog

print("Starting Program")

# dgrlin.compile("decompile")

def open_file(mode):
    if mode == "compile":
        filename = tkinter.filedialog.askopenfilename(initialdir = "./data",
                                          title = "Select a text file!",
                                          filetypes = (("Text files",
                                                        "*.txt*"),
                                                       ("all files",
                                                        "*.*")))
        
        if filename == "":
            file_open_log.configure(text="No file selected!")
            return

        dgrlin.compile(filename)

    elif mode == "decompile":
        filename = tkinter.filedialog.askopenfilename(initialdir = "./data",
                                          title = "Select a binary!",
                                          filetypes = (("Binary files",
                                                        "*.bin*"),
                                                        ("all files",
                                                        "*.*")))
        
        if filename == "":
            file_open_log.configure(text="No file selected!")
            return
        
        dgrlin.decompile(filename)
    
    file_open_log.configure(text=f"File {mode}d: {filename}")


root = Tk()
root.resizable(width=False, height=False)
root.geometry("800x500")
root.title("DGR LIN PARSER")

main_frame = ttk.Frame(root, padding = 10)
main_frame.grid()

ttk.Label(main_frame, text="Gello Gorld!").grid(column=0, row=0)
ttk.Button(main_frame, text="Compile", command=lambda: open_file("compile")).grid(column=0, row=1)
ttk.Button(main_frame, text="Decompile", command=lambda: open_file("decompile")).grid(column=1, row=1)

file_open_log = Label(main_frame, text="")
file_open_log.grid(column=0, row=2)

root.mainloop()
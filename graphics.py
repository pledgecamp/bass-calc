import matplotlib.pyplot as plt
import matplotlib
matplotlib.use("TkAgg")
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg, NavigationToolbar2TkAgg
from matplotlib.figure import Figure
from matplotlib.backend_bases import key_press_handler
import tkinter as tk
from tkinter import ttk

LARGE_FONT= ("Verdana", 12)

app = None

class App(tk.Tk):

    def __init__(self, *args, **kwargs):
        tk.Tk.__init__(self, *args, **kwargs)

        tk.Tk.wm_title(self, "Bass Visulization")

        container = tk.Frame(self)
        container.pack(side="top", fill="both", expand = True)
        container.grid_rowconfigure(0, weight=1)
        container.grid_columnconfigure(0, weight=1)

    def show_main(self, graph):
        main = MainPage(self, graph)
        main.grid(sticky="nsew")
        main.pack()
        main.tkraise()
        return main

    def exit(self):
        self.quit()     # stops mainloop
        self.destroy()  # this is necessary on Windows to prevent
                        # Fatal Python Error: PyEval_RestoreThread: NULL tstate

class MainPage(tk.Frame):

    def __init__(self, parent, graph):
        tk.Frame.__init__(self, parent)
        self.app = parent
        self.graph = graph
        label = tk.Label(self, text="Bass Calculator!", font=LARGE_FONT)
        label.pack(padx=10, pady=10)
        
        self.fs_scale = self.setup_scale("Fs ", 30, 100, 55)
        self.fp_scale = self.setup_scale("Fp ", 10, 80, 35)

        self.graph_canvas = FigureCanvasTkAgg(graph, self)
        self.graph_canvas.show()
        self.graph_canvas.get_tk_widget().pack(side=tk.TOP, fill=tk.BOTH, expand=True)

        toolbar = NavigationToolbar2TkAgg(self.graph_canvas, self)
        toolbar.update()
        #canvas._tkcanvas.grid(row=9,column=0)

        button = tk.Button(master=self, text='Quit', command=self.app.exit)
        button.pack(side=tk.BOTTOM)
        self.graph_canvas._tkcanvas.pack(side=tk.TOP, fill=tk.BOTH, expand=True)

        def on_key_event(event):
            print('you pressed %s' % event.key)
            key_press_handler(event, self.graph_canvas, toolbar)

        self.graph_canvas.mpl_connect('key_press_event', on_key_event)

    def setup_scale(self, label, from_, to, default, resolution=1):
        scaleFrame = tk.Frame(self)
        scaleFrame.pack(side=tk.TOP)
        scaleLabel = tk.Label(scaleFrame, text=label)
        scaleLabel.pack(side=tk.LEFT, anchor=tk.S)

        scale = tk.Scale(scaleFrame, from_=from_, to=to, orient=tk.HORIZONTAL, resolution=resolution)
        scale.set(default)
        scale.pack(side=tk.LEFT)
        return scale

    def set_fs_callback(self, callback):
        def changed(value):
            self.fs_scale_callback(self, value)
        
        self.fs_scale_callback = callback
        self.fs_scale.configure(command=changed)

    def set_fp_callback(self, callback):
        def changed(value):
            self.fp_scale_callback(self, value)
        
        self.fp_scale_callback = callback
        self.fp_scale.configure(command=changed)

    def update_graph(self, w, mag, phase):
        self.graph.update(w, mag, phase)
        self.graph_canvas.draw()

class BassGraph(Figure):

    def __init__(self, w, mag, phase):
        Figure.__init__(self, figsize=(8, 5), dpi=100)

        self.w = w
        self.mag = mag
        self.phase = phase

        self.plot = self.add_subplot(1, 1, 1)
        self.plot.plot(w, mag, 'b')

    def update(self, w, mag, phase):
        self.w = w
        self.mag = mag
        self.phase = phase
        self.plot.cla()
        self.plot.plot(w, mag, 'b')

def init_app():
    global app
    app = App()

def show_main(w, mag, phase):
    main = app.show_main(BassGraph(w, mag, phase))
    return main

def start_app():
    app.mainloop()

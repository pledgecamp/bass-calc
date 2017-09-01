import matplotlib.pyplot as plt
import matplotlib
matplotlib.use("TkAgg")
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg, NavigationToolbar2TkAgg
from matplotlib.figure import Figure
from matplotlib.backend_bases import key_press_handler
import tkinter as tk
from tkinter import ttk
from util import float_or_none, pairs

LARGE_FONT= ("Verdana", 12)

class App(tk.Tk):

    def __init__(self, *args, **kwargs):
        tk.Tk.__init__(self, *args, **kwargs)
        tk.Tk.wm_title(self, "Bass Visulization")

    def show_main(self, graph, param_groups):
        main = MainPage(self, graph, param_groups)
        main.pack(fill=tk.BOTH, expand=1)
        main.tkraise()
        return main

    def exit(self):
        self.quit()     # stops mainloop
        self.destroy()  # this is necessary on Windows to prevent
                        # Fatal Python Error: PyEval_RestoreThread: NULL tstate

class MainPage(tk.Frame):

    def __init__(self, parent, graph, param_groups):
        tk.Frame.__init__(self, parent)
        self.app = parent
        self.graph = graph
        
        self.param_tabs = ttk.Notebook(self)
        self.param_tabs.pack(side=tk.LEFT, fill=tk.BOTH, expand=0)

        for group in param_groups:
            self.setup_param_group(group)

        self.graph_canvas = FigureCanvasTkAgg(graph, self)
        self.graph_canvas.show()
        self.graph_canvas.get_tk_widget().pack(side=tk.TOP, fill=tk.BOTH, expand=1)
        self.graph_canvas._tkcanvas.pack(side=tk.TOP, fill=tk.BOTH, expand=1)

        toolbar = NavigationToolbar2TkAgg(self.graph_canvas, self)
        toolbar.update()

        button = tk.Button(master=self, text='Quit', command=self.app.exit)
        button.pack(side=tk.BOTTOM)

        def on_key_event(event):
            print('you pressed %s' % event.key)
            key_press_handler(event, self.graph_canvas, toolbar)

        self.graph_canvas.mpl_connect('key_press_event', on_key_event)
    
    def setup_param_group(self, group):
        frame = tk.Frame(self.param_tabs)
        frame.pack(fill=tk.BOTH, expand=1)
        self.param_tabs.add(frame, text=group.title)
        i = 0
        for p1, p2 in pairs(group.params):
            self.setup_param(frame, p1, i, 0)
            if p2:
                self.setup_param(frame, p2, i, 1)
            i += 1

    def setup_param(self, frame, param, row, col):
        value = param.get_value()
        frame = tk.Frame(frame)
        frame.grid(padx=14, pady=6, row=row, column=col)
        
        label = tk.Label(frame, text=param.get_label())
        label.grid(row=0, column=0, sticky=tk.W)
        def validate_cmd(action, text):
            if action == 'key' and (len(text) > 6):
                return False
            elif action == 'focusout':
                val = float_or_none(text)
                if (not val) or (val > param.get_max()) or (val < param.get_min()):
                    print("Focusout invalid")
                    label.configure(fg='red')
                    return False
            label.configure(fg='black')
            return True
        vcmd = (frame.register(validate_cmd), '%V', '%P')
        entry = tk.Entry(frame, width=6, validate='all', vcmd=vcmd)
        entry.insert(0, str(value)[:6])
        entry.grid(row=0, column=1)

        units = tk.Label(frame, text=param.get_units())
        units.grid(row=0, column=2, sticky=tk.W)

        scale = tk.Scale(frame, from_=param.get_min(), to=param.get_max(), showvalue=0,
                        orient=tk.HORIZONTAL, resolution=param.get_resolution())
        scale.set(value)
        scale.grid(row=1, columnspan=3)
        return frame

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

        self.plot = self.add_subplot(1, 1, 1)
        self.update(w, mag, phase)

    def update(self, w, mag, phase):
        self.w = w
        self.mag = mag
        self.phase = phase
        self.plot.cla()
        self.plot.plot(w, mag, 'b')


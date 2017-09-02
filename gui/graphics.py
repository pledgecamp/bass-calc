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
        tk.Tk.wm_title(self, "Bass Visualization")

    def show_main(self, graph, param_groups):
        main = MainPage(self, graph, param_groups)
        main.pack(fill=tk.BOTH, expand=1)
        main.tkraise()
        return main

    def exit(self):
        self.quit()     # stops mainloop
        self.destroy()  # this is necessary on Windows to prevent
                        # Fatal Python Error: PyEval_RestoreThread: NULL tstate

class ParamWidget(tk.Frame):

    def __init__(self, parent, param):
        tk.Frame.__init__(self, parent)
        self.param = param
        value = param.get_value()
        
        self.label = tk.Label(self, text=param.get_label())

        scale_cmd = self.make_scale_cmd()
        self.slider = tk.Scale(self, from_=param.get_min(), to=param.get_max(), showvalue=0,
                        command=scale_cmd, orient=tk.HORIZONTAL, resolution=param.get_resolution())
        self.ignore_slider = True

        validate_cmd = self.make_param_validator(param)
        vcmd = (self.register(validate_cmd), '%V', '%P')
        self.entry = tk.Entry(self, width=6, validate='key', vcmd=vcmd)
        self.slider.set(value)
        self.entry.insert(0, str(value)[:6])

        self.label.grid(row=0, column=0, sticky=tk.W)
        self.entry.grid(row=0, column=1)
        self.units = tk.Label(self, text=param.get_units())
        self.units.grid(row=0, column=2, sticky=tk.W)
        self.slider.grid(row=1, columnspan=3)
    
    def set_entry(self, value, set_slider=False):
        if (value > self.param.get_max()) or (value < self.param.get_min()):
            self.label.configure(fg='red')
            return
        if set_slider:
            print("Set slider {}".format(value))
            self.ignore_slider = True
            self.slider.set(value)
        else:
            self.entry.delete(0, tk.END)
            self.entry.insert(0, str(value)[:6])

    def make_param_validator(self, param):
        def validate_cmd(action, text):

            self.label.configure(fg='black')
            val = float_or_none(text)
            if len(text) > 6:
                return False
            elif val is None:
                self.label.configure(fg='red')
                return True
            else:
                self.set_entry(val, set_slider=True)
            return True
        return validate_cmd

    def make_scale_cmd(self):
        def changed(value):
            if self.ignore_slider:
                self.ignore_slider = False
                return
            fval = float_or_none(value)
            if not (fval is None):
                self.set_entry(fval)
        return changed

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
        widget = ParamWidget(frame, param)
        widget.grid(padx=14, pady=6, row=row, column=col)
        return widget

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


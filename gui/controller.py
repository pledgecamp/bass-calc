from .graphics import App, BassGraph
from calc.parameters import driver1_parameters, driver2_parameters, passive_parameters, enclosure_parameters, constant_parameters

app = None

class ParamController:

    def __init__(self, param):
        self.param = param

    def on_change(self, value):
        print("{} change to {}".format(self.get_name(), value))
    
    def set_widget(self, widget):
        self.widget = widget
    
    def get_name(self):
        return self.param.name

    def get_label(self):
        return "{}:".format(self.get_name())

    def get_value(self): # TODO -- convert to current units
        return self.param.m.real
    
    def get_units(self):
        return "{:~P}".format(self.param.to_root_units().u)

    # TODO -- convert to current units (for get_max as well)
    def get_min(self):
        return self.param.min_value.m.real
    
    def get_max(self):
        return self.param.max_value.m.real
    
    def get_resolution(self):
        return 1

    def __repr__(self):
        return "gui_{}".format(self.param.name)
    
class ParamGroup:

    def __init__(self, title, params):
        self.title = title
        self.params = [ParamController(p) for p in params]

driver1_group = ParamGroup("Driver1", driver1_parameters)
driver2_group = ParamGroup("Driver2", driver2_parameters)
passive_group = ParamGroup("Passive", passive_parameters)
enclosure_group = ParamGroup("Enclosure", enclosure_parameters)
constant_group = ParamGroup("Constant", constant_parameters)
groups = [driver1_group, driver2_group, passive_group, enclosure_group, constant_group]

def init_app():
    global app
    app = App()

def show_main(w, mag, phase):
    main = app.show_main(BassGraph(w, mag, phase), groups)
    return main

def start_app():
    app.mainloop()
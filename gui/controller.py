from .graphics import App, BassGraph
from calc.parameters import driver1_parameters, driver2_parameters, passive_parameters, enclosure_parameters, constant_parameters

app = None

class ParamController:
    param_changed_callback = None

    def __init__(self, param):
        self.param = param
        param.update_callback = self.on_update

    # Value changed from GUI
    def on_change(self, value):
        print("{} change to {}".format(self.get_name(), value))
        self.param.set_value(value, self.get_units())
        self.param.invalidate()
        self.param.update_parents()
        if not (ParamController.param_changed_callback is None):
            ParamController.param_changed_callback(self.param, value)
        check_valid()

    # Parameter value updated (likely due to cascading update) 
    def on_update(self, param):
        self.widget.set_entry(param.m.real)
        self.widget.set_entry(param.m.real, set_slider=True)
    
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
    
    def is_valid(self):
        return self.param.is_valid()

    def show_invalid(self):
        self.widget.entry.config(bg='red')

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

def check_valid():
    for group in groups:
        for param_ctrl in group.params:
            if not param_ctrl.param.is_valid():
                #print("{} invalid".format(param_ctrl.param.name))
                param_ctrl.show_invalid()


def init_app():
    global app
    app = App()

def register_param_changed_callback(callback):
    ParamController.param_changed_callback = callback

def show_main(w, mag, phase):
    main = app.show_main(BassGraph(w, mag, phase), groups)
    check_valid()
    return main

def start_app():
    app.mainloop()
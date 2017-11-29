from enum import Enum
from math import pi
from numpy import sqrt
import types
from pint.compat import string_types
from pint import UnitRegistry
ureg = UnitRegistry()
Q_ = ureg.Quantity

parameters = {}

pi2 = pi ** 2

class ParameterException(Exception):
    pass

class PState(Enum):
    INIT = 0
    VALID = 1
    INVALID = 2

    def __repr__(self):
        return self.name

class PGroup(Enum):
    CONSTANT = 0
    SPEAKER = 1
    PASSIVE = 2
    ENCLOSURE = 3

def Parameter(name, initial=0, min_value=None, max_value=None):
    if parameters.get(name):
        raise ParameterException('Cannot have multiple parameters with the same name')
    inst = Q_(initial)
    inst.state = PState.INIT
    
    parameters[name] = inst
    inst.name = name
    inst.parents = []
    inst.children = []
    inst.min_value = Q_(min_value)
    inst.max_value = Q_(max_value)

    inst.set = types.MethodType(q_set, inst)
    inst.set_to = types.MethodType(q_set_to, inst)
    inst.set_quantity_str = types.MethodType(q_set_quantity_str, inst)
    inst.add_children = types.MethodType(q_add_children, inst)
    inst.update = types.MethodType(q_update, inst)
    inst.set_quantity = types.MethodType(q_set_quantity, inst)
    inst.invalidate = types.MethodType(q_invalidate, inst)
    inst.has_range = types.MethodType(q_has_range, inst)
    inst.debug_units = types.MethodType(q_debug_units, inst)
    inst.set_update_fn = types.MethodType(q_set_update_fn, inst)
    inst.set_value = types.MethodType(q_set_value, inst)
    inst.__str__ = types.MethodType(q__str__, inst)
    inst.__repr__ = types.MethodType(q__repr__, inst)
    return inst

def q_set(self, value, min_, max_, units):
    self.set_to(Q_(value, units))
    self.min_value = Q_(min_, units)
    self.max_value = Q_(max_, units)

def q_set_to(self, other_quantity):
    self._magnitude = other_quantity.magnitude
    self._units = other_quantity._units

def q_set_value(self, value, units):
    other = value * ureg(units)
    self.set_to(other)

def q_set_quantity_str(self, value_str):
    if not isinstance(value_str, str):
        raise ParameterException('Expected string')
    self.set_quantity(Q_(value_str))

def q_add_children(self, *others):
    for other in others:
        self.children.append(other)
        other.parents.append(self)

def q_update(self):
    self.set_quantity(self.update_fn())

def q_set_update_fn(self, update_fn):
    self.update_fn = update_fn
    for child_name in update_fn.__code__.co_names:
        child = parameters.get(child_name)
        if child:
            self.children.append(child)
        elif child_name in ['pi', 'pi2', 'sqrt']:
            pass
        else:
            print("Error - unknown parameter {} in {}'s equation".format(child_name, self.name))

def q_set_quantity(self, value):
    self.set_to(value)
    self.state = PState.VALID
    for child in self.children:
        child.invalidate(self)

def q_invalidate(self, changed_parent=None):
    # Stops infinite recursion. TODO -- check if infinite recursion is a bug
    if self.state != PState.INVALID:
        self.state = PState.INVALID
        for parent in self.parents:
            if parent != changed_parent:
                parent.invalidate()
        for child in self.children:
            child.invalidate(self)
    else:
        #print(self.name)
        pass

def q_has_range(self):
    return (not (self.min_value is None)) and (not (self.max_value is None))

def q_debug_units(self, space=""):
    debug = "{}: {}{}".format(self.name, space, self.to_base_units())
    for child in self.children:
        #debug += "\n{}".format(child.debug_units(space+"  "))
        debug += "\n  {}: {}".format(child.name, child.to_base_units())
    return debug

def q__str__(self):
    return repr(self)

def q__repr__(self):
    return "({}, {}, {}, {})".format(self.name, super().__repr__(), repr(self.state), [c.name for c in self.children])

def Leaf(name, initial, min_value=None, max_value=None):
    inst = Parameter(name, initial, min_value, max_value)
    inst.update_fn = types.MethodType(lambda self: self, inst)
    return inst

# Environmental parameters
ρ0 = Leaf('ρ0', '1.1839 kg / m**3', '1 kg / m**3', '1.4 kg / m**3') # density of air at 1atm and 25C
c = Leaf('c', '345 m/s', '340 m/s', '350 m/s') # speed of sound
t = Leaf('t', '1s', '1s', '1s')

# Driver low level parameters
Xmax = Leaf('Xmax', '3mm', '0mm', '100mm')
Vd = Parameter('Vd', '0.1 liter', '0liter', '100liter')
Sd = Leaf('Sd', '10 cm ** 2', '0cm**2', '1000cm**2')   
Bl = Leaf('Bl', '1 tesla meter', '0 tesla meter', '100 tesla meter')
Re = Leaf('Re', '4 ohm', '0ohm', '1000 ohm')
Mmd = Leaf('Mmd', '1kg', '1 gram', '100kg')
Mms = Parameter('Mms', '1kg', '1 gram', '100kg')
Mas = Parameter('Mas', '1kg * cm**2', '0kg * cm**2', '1000kg * cm**2')
Rms = Leaf('Rms', '4 N * s / m', '0 N * s / m', '1000 N * s / m')
Ras = Parameter('Ras', '1 ohm', '0 ohm', '1000 ohm')
Cms = Leaf('Cms', '1 meter / N', '0 meter / N', '1000 meter / N')
Cas = Parameter('Cas', '1 meter**5 / N', '0 meter**5 / N', '100 meter**5 / N')
Vas = Parameter('Vas', '1 liter', '0 liter', '100 liter')

Rg = Leaf('Rg', '0', '0', '10000')

# Driver mid level parameters
Ts = Parameter('Ts', '0.02s', '0.2s', '(1/5000)s')
ωs = Parameter('ωs', '50Hz', '5Hz', '5kHz')
Fs = Parameter('Fs', '314.159Hz', '31.4159Hz', '31415.93Hz')
Qes = Parameter('Qes', '0.5', '0', '30')
Qms = Parameter('Qms', '0.5', '0', '30')
Qts = Parameter('Qts', '0.5', '0', '30')
Qs = Parameter('Qs', '0.5', '0', '30')
Cab = Leaf('Cab', '1 meter**5 / N', '0 meter**5 / N', '100 meter**5 / N')
Vb = Parameter('Vb', '0.1 liter', '0liter', '100liter')

# Passive radiator low level parameters
Vap = Parameter('Vap', '1 liter', '0 liter', '100 liter')
Cmp = Leaf('Cmp', '1 meter / N', '0 meter / N', '1000 meter / N')
Cap = Parameter('Cap', '1 meter**5 / N', '0 meter**5 / N', '100 meter**5 / N')
Rmp = Leaf('Rmp', '4 N * s / m', '0 N * s / m', '1000 N * s / m')
Rap = Parameter('Rap', '1 ohm', '0 ohm', '1000 ohm')
Mmp = Leaf('Mmp', '1kg', '1 gram', '100kg')
Map = Parameter('Map', '1kg / cm**2', '0kg / cm**2', '1000kg / cm**2')
Sp = Leaf('Sp', '10 cm**2', '0cm**2', '1000cm**2')

# Passive radiator mid level parameters
Qmp = Parameter('Qmp', '0.5', '0', '30')
ωp = Parameter('ωp', '20Hz', '0Hz', '1000Hz')
Fp = Parameter('Fp', '120Hz', '0Hz', '6282Hz')
Tp = Parameter('Tp', '0.05s', '0s', '0.001s')

# Enclosure parameters
ωb = Parameter('ωb', '20Hz', '0Hz', '1000Hz')
Fb = Parameter('Fb', '120Hz', '0Hz', '6282Hz')
Tb = Parameter('Tb', '0.05s', '0s', '0.001s')

α = Parameter('α', '3.0', '0', '100')
δ = Parameter('δ', '7.0', '0', '100')
y = Parameter('y', '0.5', '0', '100')
h = Parameter('h', '0.5', '0', '100')
η0 = Parameter('η0', '0.4', '0', '100')

Vd.set_update_fn(lambda: (Sd * Xmax))

Mms.set_update_fn(lambda: (Mmd + (2 * ((8 * ρ0) / (3 * pi2 * sqrt( Sd / pi )))) * (Sd ** 2)))

Mas.set_update_fn(lambda: (Mms / (Sd ** 2)))

Ras.set_update_fn(lambda: (Rms / (Sd ** 2)))

Cas.set_update_fn(lambda: (Cms * (Sd ** 2)))

Vas.set_update_fn(lambda: (ρ0 * (c**2) * Cas))

Ts.set_update_fn(lambda: (1 / ωs))
ωs.set_update_fn(lambda: (Fs * 2*pi))
Fs.set_update_fn(lambda: (1 / ( 2 * pi * sqrt(Mas * Cas))))

Qes.set_update_fn(lambda: ((ωs * Re * Mas * (Sd ** 2)) / (Bl ** 2)))
Qms.set_update_fn(lambda: (1 / (ωs * Cas * Ras)))
Qts.set_update_fn(lambda: ((Qes * Qms) / (Qes + Qms)))
Qs.set_update_fn(lambda: Qts)

Vb.set_update_fn(lambda: (ρ0 * (c**2) * Cab))

Vap.set_update_fn(lambda: (ρ0 * (c**2) * Cap))
Cap.set_update_fn(lambda: (Cmp * (Sp ** 2)))
Rap.set_update_fn(lambda: (Rmp / (Sp ** 2)))
Map.set_update_fn(lambda: (Mmp / (Sp ** 2)))

Qmp.set_update_fn(lambda: (1 / (ωp * Cap * Rap)))
Fp.set_update_fn(lambda: (1 / ( 2 * pi * sqrt(Map * Cap))))
Tp.set_update_fn(lambda: (1 / ωp))
ωp.set_update_fn(lambda: (Fp * 2*pi))

Fb.set_update_fn(lambda: sqrt( (1 + (Cab / Cap)) / (2 * pi * Cab * Map)))
Tb.set_update_fn(lambda: (1 / ωp))
ωb.set_update_fn(lambda: (Fp * 2*pi))
α.set_update_fn(lambda: (Cas / Cab))
δ.set_update_fn(lambda: (Cap / Cab))
y.set_update_fn(lambda: (Fb / Fs))
h.set_update_fn(lambda: Fb / Fp)
η0.set_update_fn(lambda: ((4 * pi ** 2) / (c ** 3)) * ((Fs ** 3) * Vas / Qes))

for param in parameters.values():
    param.update()

driver1_parameters = [
    Xmax, Vd, Sd, Bl, Re, Mmd, Mms, Mas, Rms, Ras, Cms, Cas, Vas, Rg
]
driver2_parameters = [
    Ts, ωs, Fs, Qes, Qms, Qts, Qs
]
passive_parameters = [
    Vap, Cmp, Cap, Rmp, Rap, Mmp, Map, Sp,
    Qmp, ωp, Fp, Tp
]
enclosure_parameters = [
    Vb, Cab, ωb, Fb, Tb, α, δ, y, h, η0
]
constant_parameters = [
    ρ0, c, t
]

if __name__ == '__main__':
    print(len(parameters.values()))
    print(len(driver_parameters)+len(passive_parameters)+len(enclosure_parameters)+len(constant_parameters))
    print(Xmax.m.real)
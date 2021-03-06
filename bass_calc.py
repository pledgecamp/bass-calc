#!/usr/local/bin/python
#coding: utf-8
import os, sys
import math
import numpy as np
from numpy import sqrt, power
from scipy import signal
from scipy.integrate import odeint
from gui.controller import init_app, show_main, start_app, register_param_changed_callback
from calc.parameters import parameters, Cab, Qs, Qmp, Vb, Tp, Ts, α, δ, y, Fp, Fs, t, Tb, Qms, Qes
from file_utils import load_defaults

# Equations from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator"
# by Douglas H. Hurlburt

class Equation:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Γ =  0.2 #τb / Ts # 0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return Equation.calculate_static(self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Γ, ψ):

        T0 = Ts / (sqrt(y) * power(ψ, 0.25)) #8a
        a1 = ((sqrt(y) / power(ψ,0.25)) * 
            ((1 / Qmp) + (1 / (y * Qs)) + (Γ * ((α / y) + (y * δ)))))

        a2 = ((1 / sqrt(ψ)) * (((α + 1) / y) +
                                    (y * (δ + 1)) +
                                    (1 / (Qmp * Qs)) +
                                    (Γ *((α / Qmp) +
                                    (y * (δ / Qs))))))

        a3 = ((sqrt(y) / power(ψ, 0.75)) *
            (((δ + 1) / Qs) + ((α + 1) / (y * Qmp)) + (Γ * (α + δ))))
        
        
        
        b1 = sqrt(y) / (Qmp * power(ψ, 0.25))
        b2 = y / sqrt(ψ)

        num = [1, b1, b2, 0, 0 ]
        den = [1, a1, a2, a3, 1]

        print(num)
        print(den)

        sys = signal.TransferFunction(num, den)
        print(sys)
        return signal.bode(sys)

class EquationAlternateFormulation:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Γ =  0.2 #τb / Ts # 0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return EquationAlternateFormulation.calculate_static(self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Γ, ψ):

        b4 = (Ts ** 2) * (Tp ** 2) 
        b3 = (Ts ** 2) * (Tp / Qmp) 
        b2 = (Ts ** 2)
        b1 = 0 * t
        b0 = 0

        a4 = (Ts ** 2) * (Tp ** 2)
        a3 = (Ts ** 2) * Tp / Qmp + \
                (Γ * Ts) * (α * (Tp ** 2) + (δ * (Ts ** 2))) 
        a2 = (Tp ** 2) * (α + 1) + \
                (Ts ** 2) * (δ + 1) + \
                (Ts * Tp) / (Qs * Qmp) + \
                (Γ * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qs))
        a1 = Ts * (δ + 1) / Qs + \
                Tp * (α + 1) / Qmp + \
                (Γ * Ts) * (α + δ)
        a0 = ψ
       
        
        num = [b4, b3, b2, b1, b0]
        den = [a4, a3, a2, a1, a0]

        #make num and den, unitless for signal.TransferFunction

        normalize = [(t**4), (t**3), (t**2), (t**1), 1]
        num = [x/y for x,y in zip(num,normalize)]
        den = [x/y for x,y in zip(den,normalize)]

        #num = [seq[0] for seq in num]
        #den = [seq[0] for seq in den]

        print(num)
        print(den)
        
        sys = signal.TransferFunction(num, den)
        print(sys)
        return signal.bode(sys)

class EquationDisplacementDriver:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Γ =  0.2 #τb / Ts # 0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return EquationDisplacementDriver.calculate_static(self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Γ, ψ):

        b4 = 0 * (t ** 4)
        b3 = 0 * (t ** 3)
        b2 = ψ * (Tb ** 2)  
        b1 = 0 * t
        b0 = ψ

        a4 = (Ts ** 2) * (Tp ** 2)
        a3 = (Ts ** 2) * Tp / Qmp + \
                (Γ * Ts) * (α * (Tp ** 2) + (δ * (Ts ** 2))) 
        a2 = (Tp ** 2) * (α + 1) + \
                (Ts ** 2) * (δ + 1) + \
                (Ts * Tp) / (Qs * Qmp) + \
                (Γ * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qs))
        a1 = Ts * (δ + 1) / Qs + \
                Tp * (α + 1) / Qmp + \
                (Γ * Ts) * (α + δ)
        a0 = ψ
       
        
        num = [b4, b3, b2, b1, b0]
        den = [a4, a3, a2, a1, a0]

        #make num and den, unitless for signal.TransferFunction

        normalize = [(t**4), (t**3), (t**2), (t**1), 1]
        num = [x/y for x,y in zip(num,normalize)]
        den = [x/y for x,y in zip(den,normalize)]

        #num = [seq[0] for seq in num]
        #den = [seq[0] for seq in den]

        print(num)
        print(den)
        
        sys = signal.TransferFunction(num, den)
        print(sys)
        return signal.bode(sys)


class EquationDisplacementPassive:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Γ =  0.2 #τb / Ts # 0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return EquationDisplacementPassive.calculate_static(self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Γ, ψ):

        b4 = 0 * (t ** 4)
        b3 = 0 * (t ** 3)
        b2 = 0 * (t ** 2)  
        b1 = 0 * t
        b0 = ψ

        a4 = (Ts ** 2) * (Tp ** 2)
        a3 = (Ts ** 2) * Tp / Qmp + \
                (Γ * Ts) * (α * (Tp ** 2) + (δ * (Ts ** 2))) 
        a2 = (Tp ** 2) * (α + 1) + \
                (Ts ** 2) * (δ + 1) + \
                (Ts * Tp) / (Qs * Qmp) + \
                (Γ * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qs))
        a1 = Ts * (δ + 1) / Qs + \
                Tp * (α + 1) / Qmp + \
                (Γ * Ts) * (α + δ)
        a0 = ψ
       
        
        num = [b4, b3, b2, b1, b0]
        den = [a4, a3, a2, a1, a0]

        #make num and den, unitless for signal.TransferFunction

        normalize = [(t**4), (t**3), (t**2), (t**1), 1]
        num = [x/y for x,y in zip(num,normalize)]
        den = [x/y for x,y in zip(den,normalize)]

        #num = [seq[0] for seq in num]
        #den = [seq[0] for seq in den]

        print(num)
        print(den)
        
        sys = signal.TransferFunction(num, den)
        print(sys)
        return signal.bode(sys)


class EquationImpedence:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Γ =  0.2 #τb / Ts # 0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return EquationImpedence.calculate_static(self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Γ, ψ):


        b4 = 0 * (t ** 4)
        b3 = ψ * (Tb ** 2) * Ts / Qes
        b2 = 0 * (t** 2)
        b1 = ψ * Ts / Qes
        b0 = 0

        a4 = (Ts ** 2) * (Tp ** 2)
        a3 = (Ts ** 2) * Tp / Qmp + \
                (Γ * Ts) * (α * (Tp ** 2) + (δ * (Ts ** 2))) 
        a2 = (Tp ** 2) * (α + 1) + \
                (Ts ** 2) * (δ + 1) + \
                (Ts * Tp) / (Qs * Qmp) + \
                (Γ * Ts) * ((α * Tp / Qmp) + (δ * Ts / Qms))
        a1 = Ts * (δ + 1) / Qms + \
                Tp * (α + 1) / Qmp + \
                (Γ * Ts) * (α + δ)
        a0 = ψ
       
        
        num = [b4, b3, b2, b1, b0]
        den = [a4, a3, a2, a1, a0]

        #make num and den, unitless for signal.TransferFunction

        normalize = [(t**4), (t**3), (t**2), (t**1), 1]
        num = [x/y for x,y in zip(num,normalize)]
        den = [x/y for x,y in zip(den,normalize)]

        #num = [seq[0] for seq in num]
        #den = [seq[0] for seq in den]

        print(num)
        print(den)
        
        sys = signal.TransferFunction(num, den)
        print(sys)
        return signal.bode(sys)



class EquationEfficencySmalls:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Γ =  0.2 #τb / Ts # 0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return EquationImpedence.calculate_static(self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Γ, ψ):

        kηq = Qt / Qes
        kηg = 4 * (math.pi ** 2) / (c ** 3) * (Vas / Vb) * ((fs ** 3) / (f3 ** 3)) * (1 / Qt)
        kη = kηq * kηg

        η0 = kη * (f3 ** 3) * Vb

        print(η0)

        return η0


class EquationEfficencyAdams:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Γ =  0.2 #τb / Ts # 0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return EquationImpedence.calculate_static(self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Γ, ψ):

        η0 = (Bl ** 2) * ρ0 / ((Sd ** 2) * (Mas ** 2) * 2 * Math.pi * c * Re)

        print(η0)

        return η0

main_equation = EquationImpedence()
main = None

def param_changed(param, value):
    update(main)

def update(main):
    print("update!")
    main_equation.update()
    w, mag, phase = main_equation.calculate()
    main.update_graph(w, mag, phase)

def run():
    global main
    load_defaults(parameters)
    init_app()
    w, mag, phase = main_equation.calculate()
    main = show_main(w, mag, phase)
    register_param_changed_callback(param_changed)
    start_app()

if __name__ == '__main__':
    run()
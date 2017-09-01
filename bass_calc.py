#!/usr/local/bin/python
#coding: utf-8
import os, sys
import math
import numpy as np
from numpy import sqrt, power
from scipy import signal
from scipy.integrate import odeint
from gui.controller import init_app, show_main, start_app
from calc.parameters import Cab, Qs, Qmp, Vb, Tp, Ts, α, δ, y, Fp, Fs

# Equation from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator"
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

        sys = signal.TransferFunction(num, den)
        print(sys)
        return signal.bode(sys)

main_equation = Equation()

def vb_changed(main, value):
    print("Vb updated: {}".format(value))
    Vb.set_value(0.5 + float(value), 'cm**3')
    update(main)

def fs_changed(main, value):
    Fs.set_value(int(value), 'Hz')
    update(main)

def fp_changed(main, value):
    Fp.set_value(int(value), 'Hz')
    update(main)

def update(main):
    main_equation.update()
    w, mag, phase = main_equation.calculate()
    main.update_graph(w, mag, phase)

def run():
    init_app()
    w, mag, phase = main_equation.calculate()
    main = show_main(w, mag, phase)
    #main.set_fs_callback(fs_changed)
    #main.set_fp_callback(fp_changed)
    start_app()

if __name__ == '__main__':
    run()
#!/usr/local/bin/python
#coding: utf-8
import os, sys
import math
import numpy as np
from scipy import signal
from scipy.integrate import odeint
from graphics import init_app, show_main, start_app

#Equation from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator" by Douglas H. Hurlburt


#Physical Constants
ρ0 = 1.18 # density of air (kg / m^3)
c = 345 #speed of sound (m / s) 

#Currently imcomplete ##############################

#T&S paramters
Vb = 1 #enclosed volume of box (liters)
Mas = 1 #Accoustic mass of driver including airload and coupling (grams)
Map = 1 #Accoustic mass of driver including airload and coupling (grams)
Sd = 1 #Effective surface of area of


#Box parameters
Rab = 1 
Cab = Vb / (ρ0 * (c ** 2)) #Accoustic compliance of enclosed volume 
τb = 0.7 #Rab * Cab #time constant of internal enclosure loss
#τl #time constant of internal enclosure leakage


#####################################################


#input paramters
fp = 35.0 #Resonant Frequency of Passive Radiator (hertz)
fs = 55.0 #Resonant Frequncy of Driver Hertz (hertz)

α = 3.0
δ = 7.0

Qs = 0.4 #Cas * Rras / Ts # Q of Driver 
Qp = 10.0 #Cap * Rap / Tp # Q of Passive Radiator

class Equation:

    def __init__(self):
        self.update()
    
    def update(self):
        self.Tp = (2.0 * math.pi * fp) ** 2.0
        self.Ts = (2.0 * math.pi * fs) ** 2.0
        self.y = fp/fs
        self.Γ =  0.2 #τb / Ts #0.2 is a good guesstimate
        self.ψ = α + δ + 1
    
    def calculate(self):
        return Equation.calculate_static(self.Tp, self.Ts, self.y, self.Γ, self.ψ)

    @staticmethod
    def calculate_static(Tp, Ts, y, Γ, ψ):
        T0 = Ts / (math.sqrt(y) * math.pow(ψ,0.25)) #8a
        a1 = ((math.sqrt(y) / math.pow(ψ,0.25)) * 
            ((1 / Qp) + (1 / (y * Qs)) + (Γ * ((α / y) + (y * δ)))))

        a2 = ((1 / math.sqrt(ψ)) * (((α + 1) / y) +
                                    (y * (δ + 1)) +
                                    (1 / (Qp * Qs)) +
                                    (Γ *((α / Qp) +
                                    (y * (δ / Qs))))))

        a3 = ((math.sqrt(y) / math.pow(ψ, 0.75)) *
            (((δ + 1) / Qs) + ((α + 1) / (y * Qp)) + (Γ * (α + δ))))

        b1 = math.sqrt(y) / (Qp * math.pow(ψ, 0.25))
        b2 = y / math.sqrt(ψ)

        num = [1, b1, b2, 0, 0 ]
        den = [1, a1, a2, a3, 1]

        sys = signal.TransferFunction(num, den)
        print(sys)
        return signal.bode(sys)

main_equation = Equation()

def vb_changed(main, value):
    print("Vb updated: {}".format(value))
    global Vb
    Vb = 0.5 + float(value)
    update(main)

def fs_changed(main, value):
    global fs
    fs = int(value)
    update(main)

def fp_changed(main, value):
    global fp
    fp = int(value)
    update(main)

def update(main):
    main_equation.update()
    w, mag, phase = main_equation.calculate()
    main.update_graph(w, mag, phase)

def run():
    init_app()
    w, mag, phase = main_equation.calculate()
    main = show_main(w, mag, phase)
    main.set_fs_callback(fs_changed)
    main.set_fp_callback(fp_changed)
    start_app()

if __name__ == '__main__':
    run()
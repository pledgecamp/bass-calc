#!/usr/local/bin/python
#coding: utf-8
import os, sys
import math
import numpy as np
from scipy import signal
import matplotlib.pyplot as plt
from scipy.integrate import odeint


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


#variables used in Equation
Tp = (2.0 * math.pi * fp) ** 2.0
Ts = (2.0 * math.pi * fs) ** 2.0
y = fp/fs
Γ =  0.2 #τb / Ts #0.2 is a good guesstimate
ψ = α + δ + 1 

T0 = Ts / (math.sqrt(y) * math.pow(ψ,0.25)) #8a
a1 = (math.sqrt(y) / math.pow(ψ,0.25)) * ((1 / Qp) + (1 / (y * Qs)) + (Γ * ((α / y) + (y * δ))))
a2 = (1 / math.sqrt(ψ)) * (((α + 1) / y) + (y * (δ + 1)) + (1 / (Qp * Qs)) + (Γ *((α / Qp) + (y * (δ / Qs)))))
a3 = (math.sqrt(y) / math.pow(ψ, 0.75)) * (((δ + 1) / Qs) + ((α + 1) / (y * Qp)) + (Γ * (α + δ)))
b1 = math.sqrt(y) / (Qp * math.pow(ψ, 0.25))
b2 = y / math.sqrt(ψ)

num = [1, b1, b2, 0, 0 ]
den = [1, a1, a2, a3, 1]


sys = signal.TransferFunction(num,den)
print(sys)
w, mag, phase = signal.bode(sys)
plt.figure(1)
plt.plot(w,mag,'b')
plt.show()
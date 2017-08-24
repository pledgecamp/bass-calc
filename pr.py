#!/usr/local/bin/python
#coding: utf-8
import os, sys
import math
import numpy as np
from scipy import signal
import matplotlib.pyplot as plt
from scipy.integrate import odeint





#T&S paramters
Vb = 10 #enclosed volume of box (liters)
Mas = 1 #Accoustic mass of driver including airload and coupling (grams)
Map = 1 #Accoustic mass of driver including airload and coupling (grams)
Sd = 1 #Effective surface of area of


fp = 45 #Resonant Frequency of Passive Radiator (hertz)
fs = 45 #Resonant Frequncy of Driver Hertz (hertz)


Tp = (2 * math.pi * fp) ** 2
Ts = (2 * math.pi * fs) ** 2

print("hello\n")

#Equation from "Complete Response Function and System Parameters for a Loudspeaker with Passive Radiator" by Douglas H. Hurlburt
α = 1
δ = 1
y = 1
Γ = 1
ψ = 1
Qs = 1
Qp = 1


y = fp/fs
Γ = τb / Ts
ψ = α + δ + 1

T0 = Ts / (math.sqrt(y) * math.pow(ψ,0.25)) #8a
a1 = (math.sqrt(y) / math.pow(ψ,0.25)) * ((1 / Qp) + 1 / (y * Qs) + Γ * (α/y + y * δ))
a2 = (1 / math.sqrt(ψ)) * (((α + 1) / y) + (y * (δ + 1)) + (1 / (Qp * Qs)) + (Γ *(α/Qp + y * δ/Qs)))
a3 = (math.sqrt(y) / math.pow(ψ, 0.75)) * (((δ + 1) / Qs) + ((α + 1) / (y * Qp)) + (Γ * (α + δ)))
b1 = math.sqrt(y) / (Qp * math.pow(ψ, 0.25))
b2 = y / math.sqrt(ψ)


num = [T0**4, b1 * T0 ** 3, b2 * T0 ** 2, 0, 0 ]
den = [T0**4, a1 * T0 ** 3, a2 * T0 ** 2, a3 * T0, 1]
sys = signal.TransferFunction(num,den)
print(sys)
t,y = signal.step(sys)
plt.figure(1)
plt.plot(t,y,'r')
plt.show()
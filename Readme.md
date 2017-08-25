# How to Run
Requires python3 (3.6 or later recommended)

- Install Python 3.6+ and ensure python3 is on your $PATH
- (Optional) Install [virtualenvwrapper](https://virtualenvwrapper.readthedocs.io)
- (Optional) Create a virtual environment:

```
mkvirtualenv --python=$(which python3) bass-calc
```
- Install dependencies:
```
pip install -r requirements.txt
```
- Run bass_calc.py
```
python bass_calc.py
```

## Mac OSX
On OSX the backend for matplotlib must be specified.
To do this, create a file in the ~/.matplotlib directory called "matplotlibrc"
and add the following line
```
backend: TkAgg
```
Note that this will change the matplotlib backed for all scripts.
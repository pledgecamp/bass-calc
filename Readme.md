# How to Run
Requires python3 (3.6 or later recommended)

- Install Python 3.6+ and ensure python3 is on your $PATH
- (Optional) Install [virtualenvwrapper](https://virtualenvwrapper.readthedocs.io)
- (Optional) Create a virtual environment:

```
mkvirtualenv --python=$(which python3) bass-calc
```
- Double check python3.6 is active (use `python --version`)
    - If not, try replacing `$(which python3)` in the mkvirtualenv command with the correct python3.6 location
- Install dependencies:
```
pip install -r requirements.txt
```
- Run bass_calc.py
```
python bass_calc.py
```

## Mac OSX
## Ubuntu/Debian
You may need to install the python3-tk package:
```
sudo apt-get install python3-tk
```

# Usage
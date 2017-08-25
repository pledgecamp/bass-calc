Requires python3 (version 3.6 or later recommended)

In order to run on OSX, the backend for matplotlib must be specified.
To do this, create a file in the ~/.matplotlib directory called "matplotlibrc"
and add the following line
```
backend: TkAgg
```
Note that this will change the matplotlib backed for all scripts.
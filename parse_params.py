
def float_or_none(x):
    try:
        return float(x)
    except ValueError:
        return None

def parse_file(filename, parameters):
    with open(filename, 'r') as f:
        i = 0
        for line in f.readlines():
            i += 1
            param = [x.strip() for x in line.split("=")]
            if len(param) == 2:
                value = float_or_none(param[1])
                # Overwrite the input value
                # If the parameter read from the file is formatted incorrectly add None
                #   as the value, unless the parameter already exists (don't overwrite)
                if value:
                    parameters[param[0]] = value
                elif not parameters.has_key(param[0]):
                    parameters[param[0]] = value
                    print "Skipped line {} (VALUE must be int or float): {}".format(i, param[1])
                else:
                    print "Warning line {} (VALUE must be int or float): {}".format(i, param[1])
            else:
                print "Skipped line {} (format should be PARAM=VALUE): {}".format(i, line)

if __name__ == "__main__":
    params = {}
    parse_file("test.bass", params)
    print params
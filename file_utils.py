from util import float_or_none

SAVE_FILE = "save_files/{}"

def peek_line(f, consume_fn=None):
    pos = f.tell()
    line = f.readline()
    if (line is None) or (consume_fn and consume_fn(line)):
        return (line, True)
    f.seek(pos)
    return (line, False)

def check_header(line):
    header = [x.strip() for x in line.split(',')]
    if len(header) < 4 or header[1].lower() == 'name':
        return True
    return False

def check_non_data(line):
    line = line.strip()
    if (len(line) == 0) or line[0] == '#':
        return None
    return line

def consume_non_data(f):
    while True:
        (line, consumed) = peek_line(f, consume_fn=check_non_data)
        if (line is None) or not consumed:
            break

def check_value(value_str, type_, line):
    value = float_or_none(value_str)
    if not (value is None):
        return value
    print("Error line {} ({} must be int or float): {}".format(line, type_, value))
    return None

def load_file(name):
    data = []
    with open(SAVE_FILE.format(name), 'r') as f:
        i = 0

        consume_non_data(f)
        peek_line(f, consume_fn=check_header)

        for line in f.readlines():
            i += 1
            line = check_non_data(line)
            if line is None:
                continue

            param = [x.strip() for x in line.split(',')]
            if len(param) < 4:
                print("Error line {} (missing info): {}".format(i, param[0]))
                continue

            default = check_value(param[1], 'default', i)
            if default is None:
                continue

            min_ = check_value(param[2], 'min', i)
            if min_ is None:
                continue

            max_ = check_value(param[3], 'max', i)
            if max_ is None:
                continue

            if len(param) == 4:
                units = 'dimensionless'
            else:
                units = param[4]

            data.append((param[0], default, min_, max_, units))
    return data

def load_defaults(parameters):
    data = load_file('defaults.bass')
    for (name, default, min_, max_, units) in data:
        if name in parameters:
            param = parameters[name]
            param.set(default, min_, max_, units)
        else:
            print("Error - unknown parameter {}".format(name))


def save_file(name, parameters):
    with open(SAVE_FILE.format(name), 'w') as f:
        lines = []
        for (key, value) in parameters.items():
            lines.append("{}={}\n".format(key, value))
        f.writelines(lines)

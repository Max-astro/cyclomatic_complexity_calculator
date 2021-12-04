def abc(a,b,c):
    for i in a:
        b(a)
    if c():
        if b(a) > 100:
            b(a)
        elif 100 > 0:
            b(a)
        else:
            b(a)
    else:
        b(100)
    return 100

def ex():
    try:
        fh = open("testfile", "w")
        fh.write("a")
    except IOError:
        print("b")
    else:
        print("c")
        fh.close()

def funa(a):
    b = a(3)
    def funab(f):
        return f(5)

    def funac(f):
        return f(5)
    def funcb(f):
        return funab(f)
    return funab(b)

def funb():
    while 1:
        for i in range(10):
            if 1:
                for j in range(100):
                    print(j)
                return 1
            elif 2:
                return 2
            else:
                return 3

def func():
    if 1:
        for j in range(100):
            print(j)
        return 1
    elif 2:
        return 2
    else:
        return 3

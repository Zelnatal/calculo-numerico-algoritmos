import math
def bisseção(f,epsilon,a,b):
    fa = f(a)
    fb = f(b)
    
    if fa*fb > 0:
        return ValueError()
    
    while (abs(a-b) >= epsilon and abs(fa) >= epsilon and abs(fb) > epsilon):
        x = (a+b)/2
        fx = f(x)
        if fa*fx < 0:
            b = x
            fb = fx
            continue
        if fb*fx < 0:
            a = x
            fa = fx
            continue
        return RuntimeError()
    
    if abs(a-b) < epsilon or abs(fa) < epsilon:
        return a
    if abs(fb) < epsilon:
        return b
    return RuntimeError()

if __name__ == '__main__':
    f = lambda x: math.sin(x)*x**x
    epsilon = 10**-15
    a = 6
    b = 7
    resultado = bisseção(f,epsilon,a,b)
    if isinstance(resultado,BaseException):
        match resultado:
            case ValueError():
                print("Intervalo invalido")
            case _:
                print("Deu error")
    else:
        print(f'A raiz encontrada é {resultado}')
        print(f'F({resultado}) é {f(resultado)}')
import matplotlib.pyplot as plt

def runge_kutta_2_ordem(a, b, n, f, yo):
    h = (b - a) / n
    xs = [a + i * h for i in range(n + 1)]
    ys = [yo] * (n + 1)
    yield (a, yo)
    for i, x in enumerate(xs[:-1]):
        y = ys[i]
        k1 = f(x, y)
        k2 = f(x + h, y + k1 * h)
        ys[i + 1] = y + (h / 2) * (k1 + k2)
        yield (xs[i + 1], ys[i + 1])
        
xs = []
ys = []

for x,y in runge_kutta_2_ordem(0,2,1000,lambda x,y: x,0):
    xs.append(x)
    ys.append(y)
plt.grid()
plt.plot(xs,ys)
plt.show()
import matplotlib.pyplot as plt

def euler(a,b,n,f,yo):
    h = (b-a)/n
    xs = [a+i*h for i in range(n+1)]
    ys = [yo] * (n+1)
    yield (a,yo)
    for i,x in enumerate(xs[:-1]):
        ys[i+1] = ys[i] + f(xs[i],ys[i])*(xs[i]-a)
        yield (xs[i+1],ys[i+1])
        
        
xs = []
ys = []

for x,y in euler(0,1,10,lambda x,y: y,10):
    xs.append(x)
    ys.append(y)
plt.grid()
plt.plot(xs,ys)
plt.show()
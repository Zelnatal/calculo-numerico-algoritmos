from typing import List


def fatoracao(a: List[List[float]]):
    if len(a) == 0:
        raise Exception("Matriz A não pode está vazio")
    if len(a) != len(a[0]):
        raise Exception("Matriz A precisa ser quadrada")
    for i in range(len(a)):
        for j in range(i + 1, len(a)):
            if a[i][j] != a[j][i]:
                raise Exception("Matriz A precisa ser simétrica")

    l = [[0] * len(a) for _ in range(len(a))]
    for i in range(len(a)):
        s = 0
        for k in range(i):
            s += l[i][k] ** 2
        l[i][i] = (a[i][i] - s) ** 0.5
        if l[i][i] == 0:
            raise Exception(f"O l[{i}][{i}] foi igual a zero")
        for j in range(i + 1, len(a)):
            s = 0
            for k in range(min(i, j)):
                s += l[i][k] * l[k][j]
            l[i][j] = l[j][i] = (a[i][j] - s) / l[i][i]
    return l


def calcular(a: List[List[float]], b: List[float]):
    l = fatoracao(a)
    if len(a) != len(b):
        raise Exception("O tamanho de B é diferente do tamanho de A")

    ys = [0] * len(a)
    for i in range(len(a)):
        s = 0
        for j in range(i):
            s += l[i][j] * ys[j]
        ys[i] = (b[i] - s) / l[i][i]

    xs = [0] * len(a)

    for i in range(len(a) - 1, -1, -1):
        s = 0
        for j in range(i + 1, len(a)):
            s += l[i][j] * xs[j]
        xs[i] = (ys[i] - s) / l[i][i]

    return xs


if __name__ == "__main__":
    a = [[1, 2, 3], [2, 6, 4], [3, 4, 10]]
    b = [4, 5, 5]
    try:
        xs = calcular(a, b)
        print("Os xs encontrados são")
        for x in xs:
            print(x)
    except Exception as e:
        print(e)

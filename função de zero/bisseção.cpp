#include <iostream>
#include <locale>
#include <cmath>
#include <iomanip>

double f(double x)
{
    return (pow(x, 3) + 4 * pow(x, 2) + 3 * x + 1);
}

void resposta(double x)
{
    std::cout << "A raiz é " << std::setprecision(15) << x;
}

int main()
{
    setlocale(LC_ALL, "pt_br.utf8");
    double a, b, fa, fb, epsilon;
    std::cout << "Valor da precisão: ";
    std::cin >> epsilon;
    std::cout << "Intervalo A B: ";
    std::cin >> a >> b; 

    fa = f(a);
    fb = f(b);

    if (!(fa * fb < 0))
    {
        std::cout << "Não foi possível encontrar";
        return 0;
    }

    while (!(abs(a - b) < epsilon || abs(fa) < epsilon || abs(fb) < epsilon))
    {
        double x = (a + b) / 2;
        double fx = f(x);
        if (fx * fa < 0)
        {
            b = x;
            fb = fx;
        }
        else
        {
            a = x;
            fa = fx;
        }
    }

    if (abs(a - b) < epsilon || abs(fa) < epsilon)
    {
        resposta(a);
        return 0;
    }
    if (abs(fb) < epsilon)
    {
        resposta(b);
        return 0;
    }

    std::cout << "Não foi possível encontrar";
    return 0;
}
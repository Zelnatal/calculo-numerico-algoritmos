#include <iostream>
#include <locale>
#include <cmath>
#include <iomanip>

double f(double x)
{
    return (pow(x, 3) + 4 * pow(x, 2) + 3 * x + 1);
}

void resposta(double x, int prec)
{
    std::cout << "A raiz é " << std::setprecision(prec) << x;
}

int main()
{
    setlocale(LC_ALL, "pt_br.utf8");
    int prec;
    double a, b, fa, fb;

    std::cout << "Valor da precisão: ";
    std::cin >> prec;
    std::cout << "Intervalo A B: ";
    std::cin >> a >> b; 

    fa = f(a);
    fb = f(b);

    if (!(fa * fb < 0))
    {
        std::cout << "Não foi possível encontrar";
        return 0;
    }

    double epsilon = pow(10, -prec);

    

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
        resposta(a, prec);
        return 0;
    }
    if (abs(fb) < epsilon)
    {
        resposta(b, prec);
        return 0;
    }

    std::cout << "Não foi possível encontrar";
    return 0;
}
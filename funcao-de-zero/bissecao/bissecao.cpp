#include <functional>
#include <stdexcept>
#include <cmath>
#include <locale>
#include <iostream>
#include <format>

using funcao = std::function<double(double)>;

double bissecao(funcao f, double epsilon, double a, double b)
{
    double fa = f(a);
    double fb = f(b);
    if (fa * fb > 0)
    {
        throw std::invalid_argument("O intervalo precisa cruza o eixo x");
    }

    while (abs(a - b) >= epsilon && abs(fa) >= epsilon && abs(fb) >= epsilon)
    {
        double x = (a + b) / 2;
        double fx = f(x);

        if (fx * fa < 0)
        {
            b = x;
            fb = fx;
            continue;
        }

        if (fx * fb < 0)
        {
            a = x;
            fa = fx;
            continue;
        }

        throw std::runtime_error("Erro durante o loop");
    }

    if (abs(a - b) < epsilon || abs(fa) < epsilon)
    {
        return a;
    }

    if (abs(fb) < epsilon)
    {
        return b;
    }

    throw std::runtime_error("Finalizado sem encontrar a raiz");
}

int main()
{
    setlocale(LC_ALL, "pt_br.utf8");
    funcao f = [](double x)
    {
        return (sin(x) * pow(x, x));
    };
    double epsilon = pow(10, -15);
    double a = 6;
    double b = 7;

    double resultado;

    try
    {
        resultado = bissecao(f, epsilon, a, b);
    }
    catch (const std::exception &e)
    {
        std::cerr << e.what() << '\n';
        return 0;
    }

    std::cout << std::format("A raiz encontrada é {}\n",resultado);
    std::cout << std::format("F({}) é {}\n",resultado,f(resultado));

    return 0;
}
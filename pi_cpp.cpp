#include <iostream>
int main()
{
    double D;
    double pi;
    double i = 0, sum = 0, sign = 1, precision = 100000000*2;
    for (;i < precision+1; i++) {
        D = (sign*(1/(2*i+1)));
        sum = sum + D;
        sign = -sign;
        pi = 4 * sum;
    }
    std::cout << pi;
    return 0;
}
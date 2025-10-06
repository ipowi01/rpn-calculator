#include <iostream>
#include <stack>
#include <cstdarg>
#include <windows.h>

std::stack<double> szamok;
std::string operators = "*/-+";

std::string get_input(){
    std::string input;
    std::cin >> input;
    return input;
}

double eval(double a, double b, char c){
    switch(c){
        case '*':
            return a*b;
        case '/':
            return a/b;
        case '+':
            return a+b;
        case '-':
            return a-b;
    }

}

double parser() {
    std::string input;
    do {
        std::cin >> input;
        if(std::isdigit(input[0])) szamok.push(std::stod(input));
        else if(operators.find(input) != std::string::npos && input.size() == 1) { //operátor-e
            double a = szamok.top();
            szamok.pop();
            double b = szamok.top();
            szamok.pop();
            szamok.push(eval(b, a, input[0]));
        }
        else if (input != "do"){
            std::cerr << "Invalid character" << '\n'; exit(1);
        }
    } while (input != "do");
    return szamok.top();
}

int main(){
    std::string input;
    do {
        std::cin >> input;
        if(input != "quit") std::cout << parser() << '\n';
    } while (input != "quit");

}
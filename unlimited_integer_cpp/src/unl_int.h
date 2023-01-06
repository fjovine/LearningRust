#ifndef UNL_INT_H
#define UNL_INT_H
#include <vector>
#include <string>
using namespace std;
class UnlInteger {
private:
    vector<unsigned> cifre;
    unsigned get_cifra(int i);
    void set_cifra(int i, unsigned n);
    char char_div(char num, unsigned &resto);
    string divide_by_two(string &s, unsigned &resto);
    void ones_complement();
    void twos_complement();
    void convert_and_push(string &s);
public:
    unsigned lsd();
    unsigned msd();
    bool is_negative();
    UnlInteger(string &s);
    UnlInteger(UnlInteger &other);
    void sum (UnlInteger &other);
    void sub (UnlInteger & other);
    void debug();
    string to_string();
    string to_string_hex();
};
#endif
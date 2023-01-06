#include "unl_int.h"
#include <algorithm>
#include <string>
#include <cassert>

using namespace std;

bool starts_with(const string& str, const string& prefix)
{
    return str.size() >= prefix.size() && str.compare(0, prefix.size(), prefix) == 0;
}
unsigned UnlInteger::get_cifra(int i) {
    if (i <cifre.size()) {
        return cifre[i];
    } else {
        if (is_negative()) {
            return -1;
        } else {
            return 0;
        }
    }
}

void UnlInteger::set_cifra(int i, unsigned n) {
    if (i<cifre.size()) {
        cifre[i] = n;
    } else {
        cifre.push_back(n);
    }
}

char UnlInteger::char_div(char num, unsigned &resto) {
    unsigned numeratore = (unsigned)(num - '0') + (resto) * 10;
    unsigned risultato = numeratore / 2;
    resto  = numeratore % 2;
    return (char)(risultato + '0');
}

string UnlInteger::divide_by_two(string &s, unsigned &resto) {
    string result = "";
    resto = 0;
    bool is_first = true;
    for (char& c : s ) 
    {
        assert((c>='0') && (c<='9'));
        if (is_first) {
            if (c > '2') {
                result.push_back(char_div(c, resto));
            } else {
                resto = 1;
            }
            is_first = false;
            continue;
        }

        result.push_back(char_div(c, resto));
    }

    return result;
}

void UnlInteger::ones_complement() {
    for (unsigned &c : cifre) {
        c = ~c;
    }
}

void UnlInteger::twos_complement() {
    bool carry = false;
    bool is_first = false;
    for (unsigned &c: cifre) {
        unsigned long partial = ~c;
        if (is_first || carry) {
            partial += 1;
        }
        c = (unsigned)(0xFFFFFFFFFFFFFFFF & partial);
        carry = (partial >> sizeof(unsigned)) > 0;

    }
}

unsigned UnlInteger::msd()
{
    assert(cifre.size() > 0);
    return cifre[cifre.size()-1];
}

unsigned UnlInteger::lsd() {
    return cifre[0];
}

bool UnlInteger::is_negative()
{
    return msd() & (1 << (sizeof(unsigned)-1));
}

void UnlInteger::convert_and_push(string &s) {
    unsigned long result = stoul(s, 0, 16);
    cifre.push_back((unsigned)result);
}

UnlInteger::UnlInteger(string &s) {
    unsigned cifra_corrente = 0;
    string numero = "";
    string digits;
    bool is_negative = false;
    if ( starts_with(s, "0x") || starts_with(s,"0X0")) {
        // hexadecimal
        int i=0;
        numero = s.substr(2);
        reverse(numero.begin(), numero.end());
        string digits;
        for (char &c : numero) {
            digits.insert(0, 1, c);
            i++;
            if (i>=16) {
                convert_and_push(digits);
                digits.clear();
                i=0;
            }
        }
        if (i>0) {
            convert_and_push(digits);
        }
    }
}

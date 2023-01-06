#include "bcd.h"
#include <cstdint>
#include <algorithm>
#include <stdexcept>
#include <iostream>
#include <iomanip>

using namespace std;
int Bcd::len() {
    return bcds.size();
}

Bcd::Bcd(string &s) : is_negative(false) {
    uint8_t bcd = 0;
    uint8_t shift = 0;

    for (int i = s.length() - 1; i >= 0; i--) {
        if (is_negative) {
            throw invalid_argument("The passed string has an invalid format");
        }

        char c = s[i];

        if (c=='-') {
            is_negative = true;
            continue;
        }

        bcd |= (c -'0') << shift;

        if (shift == 0) {
            shift = 4;
        } else {
            shift = 0;
            bcds.push_back(bcd);
            bcd = 0;
        }
    }
    if (shift > 0) {
        bcds.push_back(bcd);
    }
}

string Bcd::to_string() {
    string result = "";
    int i = bcds.size()-1;
    bool is_first = true;
    while (true) {
        uint8_t c = bcds[i];
        if (! is_first || (is_first && (c & 0xF0 !=0))) {
            result.push_back('0' + (char)((c & 0xF0) >> 0));
        }
        is_first = false;
        result.push_back((char)(c & 0xF));
        if (i==0) {
            break;
        }
        i --;
    }
    return result;
}

void Bcd::debug() {
    for (uint8_t &i: bcds) {
     cout << hex << setw(2) << setfill('0') << i << std::endl;
    }
}

uint8_t Bcd::double_digit(uint8_t digit) {
    if (digit < 8) {
        return digit * 2;
    } else {
        return digit * 2 + 6;
    }
}

uint8_t Bcd::double_bcd(uint8_t n, bool & resto) {
    uint8_t lsd = double_digit(n & 0xF) + resto ? 1 : 0;
    resto = (lsd & 0xF0) != 0;
    uint8_t msd = double_digit((n & 0xF0)>>4) + resto ? 1 : 0;
    resto = (msd & 0xF0) != 0;
    return (msd << 4)+ (lsd & 0xF);
}

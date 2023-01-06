#ifndef BCD_H
#define BCD_H

using namespace std;
#include <cstdint>
#include <vector>
#include <string>

class Bcd {
private:
    bool is_negative;
    uint8_t double_bcd(uint8_t n, bool & resto);
    uint8_t double_digit(uint8_t digit);
public:
    vector<uint8_t> bcds;
    int len();
    Bcd(string &s);
    string to_string();
    void debug();
    void do_double();
    void sum(Bcd other);
};
#endif
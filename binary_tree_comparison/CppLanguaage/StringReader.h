#ifndef STRINGREADER_H
#define STRINGREADER_H
#include <string>
using namespace std;

class StringReader {
private:
    int current_index;
    string content;

public:
    StringReader(string s);
    void print();
    char next();
    void accept(char);
    string get_next_quoted_string();
};
#endif
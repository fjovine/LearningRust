#include "StringReader.h"
#include <iostream>
#include <string>
#include <assert.h>
using namespace std;

StringReader::StringReader(string s)
{
    current_index = 0;
    content = s;
}

void StringReader::print()
{
    cout << content;
}

char StringReader::next() {
    char result = EOF;
    while (current_index < content.length()) {
        char c = content[current_index];
        current_index++;
        if (! isspace(c))
        {
            result = c;
            break;
        }
    }
    return result;
}

void StringReader::accept(char c) {
    char cc = next();
    if (cc != c) {
        cout << "Accept : atteso (" << c << ") trovato (" << cc <<")\n";
    }
    assert(cc == c);
}

string StringReader::get_next_quoted_string()
{
    string result = "";
    char c;
    while ((c=next()) != EOF) {
        if (c != '"') {
            result.push_back(c);
        } else {
            break;
        }
    }

    return result;
}
#include <iostream>
#include <string>
#include <assert.h>
#include "StringReader.h"
using namespace std;

int main ()
{

    // Unit tests for StringReader
#if true
    /******/
    cout << "Next works well:";
    StringReader sr("0123   45 67    89     ");
    int i = 0;
    while (true) {
        char c = sr.next();
        if (c ==EOF) {
            break;
        }

        assert(c==(char)(i+'0'));
        i++;
    }
    cout << "OK\n";

    /******/
    cout << "get_next_quoted_string_works_well:";
    StringReader dut("asfdas \"this_is_a_string\" asdfasd");
    while (true) {
        char c = dut.next();
        if (c == EOF) {
            break;
        }
        if (c == '"') {
            string s = dut.get_next_quoted_string();
            assert(s == "this_is_a_string");
        }
    }
    cout << "OK\n";

#endif
}
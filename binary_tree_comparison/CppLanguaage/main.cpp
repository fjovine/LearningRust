#include <iostream>
#include <string>
#include <assert.h>
#include "StringReader.h"
#include "Node.h"
using namespace std;

void decode_works_well_check(string model, string expected) {
    Node * root = Node::create_from(model);
    string found = root->to_string();
    if (expected != found) {
        cout << "model [" << model << "] expected [" << expected << "] found [" << found << "]\n";
    }
    assert(expected == found);
}

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

#if true
    /******/
    cout << "to_string works well ";

    Node * root = new Node("AA");
    Node * right = new Node("RR");
    Node * left = new Node("LL");
    root->left = left;
    root->right = right;

    assert(R"S(("AA",("LL",0,0),("RR",0,0)))S" == root->to_string());

//    cout << root->to_string();
    cout << "OK\n";
#endif

#if true
    /******/
    cout << "Decode works well:";
    decode_works_well_check("(\"AA\", 0  ,    0)","(\"AA\",0,0)");
    decode_works_well_check("(\"AA\", (\"BB\",0,0)  ,    0)","(\"AA\",(\"BB\",0,0),0)");
    decode_works_well_check(
            R"SS(("A",
                ("B",0,0),
                ("C",
                  ("D",0,0),
                  ("E",
                      ("F",0,0),
                      ("G",0,0))))))SS",
            "(\"A\",(\"B\",0,0),(\"C\",(\"D\",0,0),(\"E\",(\"F\",0,0),(\"G\",0,0))))"); 
    cout << "OK\n";
#endif
}
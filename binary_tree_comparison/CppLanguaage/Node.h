#ifndef NODE_H
#define NODE_H
#include <string>
#include "StringReader.h"
using namespace std;

class Node {
private:
    static Node * decode (StringReader * reader);
public:
    string payload;
    Node * left;
    Node * right;

    Node (string s);
    void print(int left_indent);
    string to_string();
    static Node * create_from(string s);
};

#endif
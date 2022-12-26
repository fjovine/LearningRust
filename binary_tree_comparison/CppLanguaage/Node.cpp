#include "Node.h"
#include "StringReader.h"
#include "_assert.h"
#include <iostream>
#include <string>
using namespace std;

Node::Node(string s) {
    payload = s;
    left = NULL;
    right = NULL;
}

void Node::print(int left_indent) {
    string indentation(left_indent, ' ');
    cout << indentation << payload << '\n';
    if (left != NULL) {
        left->print(left_indent+2);
    } else {
        cout << indentation << 0 << '\n';
    }

    if (right != NULL) {
        right->print(left_indent+2);
    } else {
        cout << indentation << 0 << '\n';
    }
}

string Node::to_string() {
    string result = "(\"";
    result += payload;
    result += "\",";
    if (left != NULL) {
        result += left->to_string();
    } else {
        result += "0";
    }
    result += ",";
    if (right != NULL) {
        result += right->to_string();
    } else {
        result += "0";
    }
    result += ")";
    return result;
}

Node * Node::decode(StringReader * reader) {
    reader->accept('"');
    string s = reader->get_next_quoted_string();
    Node * result = new Node(s);
    reader->accept(',');
    char c = reader->next();
    ASSERT (c!= EOF, "Invalid char 1");
    if (c == '(') {
        result->left = decode(reader);
    } else {
        ASSERT(c=='0', "Invalid char 2");
    }
    reader->accept(',');

    c = reader->next();
    ASSERT(c!= EOF, "Invalid char 3");
    if (c == '(') {
        result->right = decode(reader);
    } else {
        if (c != '0') {
            cout << "Received "<< c << ",\n";
        }
        ASSERT(c=='0', "Invalid char 4");
    }
    reader->accept(')');

    return result;
}

Node * Node::create_from(string s) {
    StringReader reader(s);
    reader.accept('(');
    return decode(&reader);
}
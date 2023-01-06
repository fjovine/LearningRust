#include <iostream>
#include <cassert>
#include <string>
#include "unl_int.h"

using namespace std;
int main()
{
    cout << "Hello world\n";
}

void assert_eq(bool expected, bool actual)
{
    assert(expected==actual);
}

void assert_eq(string &a, string &b)
{
    assert(a.compare(b) == 0);
}

void new_check_case(string &a, bool expected_sign, string & expected_result) {
  UnlInteger uInf(a);
  cout << uInf.to_string();
  //assert_eq(expected_sign, uInf.is_negative());
  //assert_eq(expected_result, uInf.to_string());
}
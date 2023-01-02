#include "unl_int.h"
#include <cassert>
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

bool UnlInteger::is_negative()
{
    return msd() & (1 << (sizeof(unsigned)-1));
}

unsigned UnlInteger::msd()
{
    assert(cifre.size() > 0);
    return cifre[cifre.size()-1];
}
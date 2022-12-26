#ifndef _ASSERT_H
#define _ASSERT_H
#include <assert.h>
#define ASSERT(c,m) if (! (c)) { cout << "Assertion failed: " << m << "\n"; assert(c); }
#endif
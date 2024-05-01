#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

uintptr_t libmusc_add(uintptr_t left, uintptr_t right);

void test_alt(Alteration a);

} // extern "C"

#include "snoopy/par_obl_primitives.h"

namespace snoopy::decl {

std::mutex m;
std::condition_variable cv;
ThreadState state;

}  // namespace snoopy::decl

#include <vector>
#include <cstring>
#include <oht/lib.h>

using namespace oht;

constexpr size_t kB = 30;
constexpr size_t kZ = 5;
static std::vector<uint8_t> kPrfKey{0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

int main() {
  std::vector<Elem<4, 4>> elems{};
  elems.reserve(kB * kZ);
  for (auto i = 0; i < kB; ++i) {
    for (auto j = 0; j < kZ; ++j) {
      uint32_t key = i * kZ + j;
      uint32_t val = key;
      Elem<4, 4> elem{};
      std::memcpy(elem.key, &key, 4);
      std::memcpy(elem.val, &val, 4);
      elems.push_back(std::move(elem));
    }
  }

  Oht<4, 4> oht(kB, kZ);
  for (const auto &elem : elems) {
    auto e = elem;
    oht.prepare(std::move(e));
  }

  oht.build(kPrfKey, 1);
  return 0;
}

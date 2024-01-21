#include "oht/crypto.h"
#include <openssl/evp.h>
#include <openssl/hmac.h>

namespace oht::decl {

uint32_t PrfInt(const PrfKey &key, const uint8_t *data, size_t data_size, uint32_t max_plus1) {
  unsigned char res[32];
  size_t res_size;
  EVP_Q_mac(nullptr, "HMAC", nullptr, "sha1", nullptr, key.data(), key.size(), data, data_size, res, 32, &res_size);
  const auto res_uint = *reinterpret_cast<const uint32_t *>(res);
  return res_uint % max_plus1;
}

}  // namespace oht::decl

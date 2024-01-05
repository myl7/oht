# oht

Oblivious 2-tier hash table

## References

- Dauterman, Emma, Vivian Fang, Ioannis Demertzis, Natacha Crooks, and Raluca Ada Popa. ["Snoopy: Surpassing the scalability bottleneck of oblivious storage."][snoopy] In _Proceedings of the ACM SIGOPS 28th Symposium on Operating Systems Principles_, pp. 655-671. 2021.
- Chan, T-H. Hubert, Yue Guo, Wei-Kai Lin, and Elaine Shi. ["Oblivious hashing revisited, and applications to asymptotically efficient ORAM and OPRAM."][oht-paper] In _Advances in Cryptology–ASIACRYPT 2017: 23rd International Conference on the Theory and Applications of Cryptology and Information Security, Hong Kong, China, December 3-7, 2017, Proceedings, Part I 23_, pp. 660-690. Springer International Publishing, 2017.

[snoopy]: https://github.com/ucbrise/snoopy
[oht-paper]: https://eprint.iacr.org/2017/924.pdf

## Feature flags

`gpl` enables GPL-licensed code.

`avx2` uses AVX2 to improve performance.
Because the included AVX2 code is GPL-licensed, if you do not enable `gpl`,
you have to provide another AVX2 wrapper implementation which can be used like:

```cpp
#ifdef USE_AVX2
#include <intrinsics/immintrin.h>
#endif
```

## License

Copyright (C) myl7

SPDX-License-Identifier: Apache-2.0

Code in `include` is licensed under SPDX-License-Identifier: GPL-3.0-or-later
To use it you need to explicitly enable a flag.
See the [feature flags](#feature-flags) section for details.

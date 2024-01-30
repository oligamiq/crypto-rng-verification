# Crypto
! Information Missing
## rc4 (Arc4)
PRNG
https://github.com/jmesmon/arc4 MIT or Apache

## chacha20
PRNG
https://github.com/rust-random/rand/tree/master/rand_chacha MIT or Apache

## Yarrow
PRNG
// https://gitlab.com/sequoia-pgp/nettle-rs <br>
// https://github.com/Cameleon00722/horizon/blob/master/src/prng.rs MIT <br>
https://github.com/oligamiq/yarrow-rs MIT or Apache
※nettle has Yarrow pRNG

## mersenne-twister
PRNG
// https://docs.rs/rustmt/latest/rustmt <br>
// https://docs.rs/mt19937/latest/mt19937 <br>
https://github.com/mztikk/mersenne-twister MIT

## shishua
PRNG
https://github.com/dbartussek/shishua_rs None

## romu
PRNG
https://github.com/hasenbanck/romu Apache

## AES128
PRNG
https://github.com/tf-encrypted/aes-prng Apache

## xoshiro256**
PRNG
Not use cryptographic but fast
https://github.com/rust-random/rngs/blob/master/rand_xoshiro MIT or Apache

## xorshift128+
PRNG
Not use cryptographic but fast
https://github.com/astocko/xorshift CC0

## Fortuna
PRNG
https://github.com/quininer/fungtaai MIT

## splitmix64
PRNG
https://github.com/astocko/xorshift CC0

## Lehmer (Park–Miller)
PRNG
https://github.com/kenOfYugen/park-miller-carta-prng MIT

## hc-128
PRNG
https://github.com/rust-random/rngs/tree/master/rand_hc MIT or Apache

## PCG
PRNG
// https://docs.rs/oorandom/latest/oorandom/ <br>
https://docs.rs/nanorand/latest/nanorand/rand/pcg64 Zlib

## Ranlux++
Not use cryptographic but fast
RNG
https://github.com/oligamiq/ranluxpp-rs LGPL

## ISAAC
Fine quality
PRNG
https://github.com/rust-random/rngs/tree/master/rand_isaac MIT or Apache

## LCG (minstd, PcgXsh64LcgRng)
raw quality and fast
PRNG
https://github.com/pitdicker/small-rngs None

## SFC64
PRNG
Not use cryptographic but fast
// https://github.com/sevmeyer/prng <br>
https://github.com/pitdicker/small-rngs None

## Jitter
FromEntropy
https://github.com/rust-random/rngs/tree/master/rand_jitter MIT or Apache

## randen
https://github.com/jedisct1/randen-rng

## Philox
PRNG
https://github.com/dominikwerder/philox

## WyRand
Not use cryptographic
PRNG
https://docs.rs/nanorand/latest/nanorand/rand/wyrand/struct.WyRand.html

## GJrand
PRNG
https://github.com/pitdicker/small-rngs

## JSF32
PRNG
https://github.com/pitdicker/small-rngs

## Msws
PRNG
https://github.com/pitdicker/small-rngs

## Sapparot32
PRNG
https://github.com/pitdicker/small-rngs

## Velox3b
PRNG
https://github.com/pitdicker/small-rngs

## LFG
PRNG
https://en.wikipedia.org/wiki/Lagged_Fibonacci_generator

## ACORN
PRNG
Not use cryptographic
https://github.com/auronandace/acorn_prng

## Rule30
PRNG
https://github.com/chloe0x0/R30-rs MIT

## rabbit
https://github.com/RustCrypto/stream-ciphers/blob/master/rabbit
https://www.researchgate.net/publication/328582597_Pseudorandom_Number_Generator_Using_Rabbit_Cipher
library not found

## knuth_b
https://cpprefjp.github.io/reference/random/knuth_b
library not found

## x86 RD Rand (Hardware Random Number Generator)
https://github.com/DKenefake/SmallPRNG
library not found

## RanQ1
PRNG
near Xorshift
https://en.wikipedia.org/wiki/Xorshift#Variations
library not found

## JLKISS
PRNG
// https://en.wikipedia.org/wiki/KISS_(algorithm) <br>
https://github.com/bhickey/librng
library not found

## MRG32k3a
PRNG
https://github.com/vigna/MRG32k3a
https://github.com/ROCm/rocRAND
library not found

## MRG31k3p
PRNG
https://github.com/ROCm/rocRAND
library not found

## BCryptGenRandom
PRNG
from entropy
only windows api
library not found

## salsa20 ×
https://crypto.stackexchange.com/questions/66640/salsa20-as-a-prng-with-streams
Not exist

## rand seed (OS rng)
https://github.com/KizzyCode/crypto_api_osrandom

# ref
- https://github.com/RustCrypto/stream-ciphers
- https://gitlab.com/sequoia-pgp/nettle-rs
- https://github.com/0xricksanchez/hantu
- https://www.pcg-random.org/posts/some-prng-implementations.html
- https://arvid.io/2018/07/02/better-cxx-prng/
- https://docs.rs/lattice-qcd-rs/latest/lattice_qcd_rs/
- https://www.pcg-random.org/
- https://en.wikipedia.org/wiki/KISS_(algorithm)
- https://github.com/google/randen
- https://numpy.org/doc/stable/reference/random/bit_generators/philox.html
- https://www.pcg-random.org/other-rngs.html
- https://github.com/ferhatgec/prandom

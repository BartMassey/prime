# prime — Miller-Rabin primality tester
Bart Massey

This Rust program will quickly give a probabilistic test for
primality. This is just Miller-Rabin, not Lucas-Lehmer, but
it is good enough for most purposes.

The program will answer `true` for all primes, and `false`
with a very high probability for composites. Specifically,
the probability of false-composite for a randomly-chosen
number is bounded by `2**-d`, where `d` is a parameter of
the program and defaults to 1024.

## Notes

The file `pp387.txt` in this distribution contains a
pseudoprime that will often fail when `d` is 1.  It was
obtained from the paper *Constructing Carmichael Numbers
which are Strong Pseudoprimes to Several Bases* by François
Arnault, published in Journal of Symbolic Computation volume
20, issue 2, August 1995, pages 151-161.

This program started life as an example I wrote in 1999 for
the [Nickle](http://nickle.org) programming language. In the
process of translating it to Rust I found and corrected a
paper-bag bug in the Nickle version.

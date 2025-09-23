/* CHOICE macro
 * Choice(e, f, g) := (e & f) ^ (~e & g)
 * Returns bits from f when e=1, bits from g when e=0
 */
#define CHOICE(e, f, g) (((e) & (f)) ^ (~(e) & (g)))

/* MEDIAN macro
 * Median(e, f, g) := (e & f) ^ (e & g) ^ (f & g)
 * Majority of bits among e, f, g
 */
#define MEDIAN(e, f, g) (((e) & (f)) ^ ((e) & (g)) ^ ((f) & (g)))

/* ROTATE macro
 * Right rotate x by n bits (assuming 64-bit values)
 */
#define ROTATE(x, n) (((x) >> (n)) | ((x) << (64 - (n))))

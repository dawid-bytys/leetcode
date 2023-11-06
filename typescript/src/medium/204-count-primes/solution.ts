/*
 * Time: O(n * log(log(n)))
 * Space: O(n)
 */
function countPrimes(n: number) {
  if (n <= 2) {
    return 0;
  }

  const primes = new Uint8Array(n);
  let count = 1;

  for (let i = 3; i < n; i += 2) {
    if (!primes[i]) {
      ++count;

      for (let j = i * i; j < n; j += 2 * i) {
        primes[j] = 1;
      }
    }
  }

  return count;
}

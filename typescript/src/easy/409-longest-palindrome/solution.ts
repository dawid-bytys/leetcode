/*
 * Time: O(n)
 * Space: O(n)
 */
function longestPalindrome(s: string) {
  const occurrencesMap = new Map<string, number>();

  for (const character of s) {
    occurrencesMap.set(character, (occurrencesMap.get(character) || 0) + 1);
  }

  let oddCount = 0;

  for (const occurrence of occurrencesMap.values()) {
    if (occurrence % 2 !== 0) {
      ++oddCount;
    }
  }

  if (oddCount > 1) {
    --oddCount;
  } else {
    oddCount = 0;
  }

  return s.length - oddCount;
}

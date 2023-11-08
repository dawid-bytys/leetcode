/*
 * Time: O(2n)
 * Space: O(n)
 */
function mostCommonWord(paragraph: string, banned: string[]) {
  const occurrencesMap: Map<string, number> = new Map();
  const words = paragraph.toLowerCase().split(/\W+/);
  let currentFreqCount = Number.MIN_SAFE_INTEGER;
  let currentFreqWord = '';

  for (const word of words) {
    if (!banned.includes(word) && word !== '') {
      occurrencesMap.set(word, (occurrencesMap.get(word) || 0) + 1);
    }
  }

  for (const [key, value] of occurrencesMap) {
    if (value > currentFreqCount) {
      currentFreqCount = value;
      currentFreqWord = key;
    }
  }

  return currentFreqWord;
}

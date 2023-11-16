/*
 * Time: O(n)
 * Space: O(1)
 */
function mostWordsFound(sentences: string[]) {
  let maxWordsCount = 0;

  for (const sentence of sentences) {
    const wordsCount = sentence.split(' ').length;
    maxWordsCount = Math.max(maxWordsCount, wordsCount);
  }

  return maxWordsCount;
}

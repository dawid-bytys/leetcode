/*
 * Time: O(n * m * log(m))
 * Space: O(n)
 */
function groupAnagrams(strs: string[]) {
  const anagramMap: Map<string, string[]> = new Map();

  for (const word of strs) {
    const sortedWord = word.split('').sort().join('');
    const foundKey = anagramMap.get(sortedWord);

    if (foundKey) {
      foundKey.push(word);
    } else {
      anagramMap.set(sortedWord, [word]);
    }
  }

  return Array.from(anagramMap.values());
}

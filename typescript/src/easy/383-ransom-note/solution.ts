/*
 * Time: O(m + n)
 * Space: O(m)
 */
function canConstruct(ransomNote: string, magazine: string) {
  const map = new Map<string, number>();

  for (let i = 0; i < magazine.length; ++i) {
    const currentChar = magazine.charAt(i);
    map.set(currentChar, (map.get(currentChar) || 0) + 1);
  }

  for (let i = 0; i < ransomNote.length; ++i) {
    const currentChar = ransomNote.charAt(i);
    const charCount = map.get(currentChar);

    if (!charCount) {
      return false;
    }

    map.set(currentChar, charCount - 1);
  }

  return true;
}

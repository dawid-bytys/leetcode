/*
 * Time: O(n * 2^n)
 * Space: O(n * 2^n)
 */
function subsets(nums: number[]) {
  const powerSet: number[][] = [];
  const powerSetSize = Math.pow(2, nums.length);

  for (let i = 0; i < powerSetSize; ++i) {
    const tempSet: number[] = [];
    const binary = i.toString(2).padStart(nums.length, '0');

    for (let j = 0; j < nums.length; ++j) {
      if (binary.charAt(j) === '1') {
        tempSet.push(nums[j]);
      }
    }

    powerSet.push(tempSet);
  }

  return powerSet;
}

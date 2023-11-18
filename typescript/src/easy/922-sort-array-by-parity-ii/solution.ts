/*
 * Time: O(n)
 * Space: O(n)
 */
function sortArrayByParityII(nums: number[]) {
  const result: number[] = Array.from({ length: nums.length });
  let currentEven = 0;
  let currentOdd = 1;

  for (let i = 0; i < nums.length; ++i) {
    const currentNum = nums[i];

    if (currentNum % 2 === 0) {
      result[currentEven] = currentNum;
      currentEven += 2;
    } else {
      result[currentOdd] = currentNum;
      currentOdd += 2;
    }
  }

  return result;
}

/*
 * Time: O(n)
 * Space: O(n)
 */
function isSameTree(p: TreeNode | null, q: TreeNode | null): boolean {
  const leftNodes = [];
  const rightNodes = [];

  function traversal(node: TreeNode | null, nodes: number[]) {
    if (node === null) {
      return;
    }

    if (node.left === null && node.right !== null) {
      nodes.push(null);
    }

    nodes.push(node.val);
    traversal(node.left, nodes);
    traversal(node.right, nodes);
  }

  traversal(p, leftNodes);
  traversal(q, rightNodes);

  return String(leftNodes) === String(rightNodes);
}

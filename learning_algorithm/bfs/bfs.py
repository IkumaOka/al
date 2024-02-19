# https://zero2one.jp/learningblog/breadth-first-depth-first-search/
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None) -> None:
        self.val = val
        self.left = left
        self.right = right

def make_tree(tree, node, i, n):
    if n > i:
        if tree[i] is None:
            return None
        tmp = TreeNode(tree[i])
        node = tmp
        node.left = make_tree(tree, node.left, 2 * i + 1, n)
        node.right = make_tree(tree, node.right, 2 * i + 2, n)
    return node

tree = [1, 2, 3, 4, 5, 6]
root = make_tree(tree, None, 0, len(tree))
ans = []
queue = deque([root])
while queue:
    node = queue.popleft()
    ans.append(node.val)
    if node.left:
        queue.append(node.left)
    if node.right:
        queue.append(node.right)
print(ans)
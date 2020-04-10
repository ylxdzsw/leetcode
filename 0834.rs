// An undirected, connected tree with N nodes labelled 0...N-1 and N-1 edges are given.

// The ith edge connects nodes edges[i][0] and edges[i][1] together.

// Return a list ans, where ans[i] is the sum of the distances between node i and all other nodes.

// Example 1:

// Input: N = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
// Output: [8,12,6,10,10,10]
// Explanation: 
// Here is a diagram of the given tree:
//   0
//  / \
// 1   2
//    /|\
//   3 4 5
// We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
// equals 1 + 1 + 2 + 2 + 2 = 8.  Hence, answer[0] = 8, and so on.
// Note: 1 <= N <= 10000

#[derive(Debug, Clone)]
struct Node {
    pub neighbours: Vec<usize>,
    pub sum: i32, // the sum of distance
    pub count: i32 // the number of nodes in its subtree
}

fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut tree = vec![Node { neighbours: vec![], sum: 0, count: 1 }; n as _];
    for edge in edges {
        tree[edge[0] as usize].neighbours.push(edge[1] as _);
        tree[edge[1] as usize].neighbours.push(edge[0] as _);
    }

    // set the subtree count, and also calculate the sum of distance to its subtree
    fn _calc_subtree(tree: &mut [Node], node: usize, parent: usize) {
        for child in tree[node].neighbours.clone() {
            if child != parent {
                _calc_subtree(tree, child, node);
                tree[node].count += tree[child].count;
                tree[node].sum += tree[child].sum + tree[child].count
            }
        }
    }

    // the sum of the root is already the distance to all nodes. For other nodes, the distance can be calculated using its parents sum, becuase there is N - x.count nodes it need to pass its parent to reach.
    fn _calc_answer(tree: &mut [Node], n: i32, node: usize, parent: usize) {
        for child in tree[node].neighbours.clone() {
            if child != parent {
                tree[child].sum = tree[node].sum - tree[child].count + n - tree[child].count;
                _calc_answer(tree, n, child, node)
            }
        }
    }

    _calc_subtree(&mut tree, 0, n as usize + 1);
    _calc_answer(&mut tree, n, 0, n as usize + 1);
    tree.into_iter().map(|Node { sum, .. }| sum).collect()
}

fn main() {
    assert_eq!(sum_of_distances_in_tree(6, vec![
        vec![0,1], vec![0,2], vec![2,3], vec![2,4], vec![2,5]
    ]), &[8,12,6,10,10,10])
}

// faster than 100.00%, less memory than 100.00%.
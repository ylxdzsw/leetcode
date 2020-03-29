// The n-queens puzzle is the problem of placing n queens on an nÃn chessboard such that no two queens attack each other.

// Given an integer n, return the number of distinct solutions to the n-queens puzzle.

// Example:

// Input: 4
// Output: 2
// Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
// [
//  [".Q..",  // Solution 1
//   "...Q",
//   "Q...",
//   "..Q."],

//  ["..Q.",  // Solution 2
//   "Q...",
//   "...Q",
//   ".Q.."]
// ]

struct Node {
    pub n: (usize, usize), // (i, j). For normal node, this is the position of a queen. For header (column) nodes, i=1 means the column must be covered, while j is used to record how many nodes is linked to it vertically
    pub l: *mut Node,
    pub r: *mut Node,
    pub u: *mut Node,
    pub d: *mut Node,
    pub c: *mut Node
}

#[allow(clippy::zero_ptr)]
impl Node {
    pub fn new() -> *mut Node {
        let node = Box::new(Node { n: (0, 0), l: 0 as *mut Node, r: 0 as *mut Node, u: 0 as *mut Node, d: 0 as *mut Node, c: 0 as *mut Node });
        Box::leak(node) as _
    }
}

macro_rules! cfor {
    ($init: stmt; $cond: expr; $step: expr; $body: block) => {{
        $init
        while $cond {
            $body; $step
        }
    }}
}

unsafe fn total_n_queens(n: i32) -> i32 {
    unsafe fn cover(col: *mut Node) { // col must be a column (header) node
        (*(*col).r).l = (*col).l;
        (*(*col).l).r = (*col).r;

        cfor!(let mut i = (*col).d; i != col; i = (*i).d; {
            cfor!(let mut j = (*i).r; j != i; j = (*j).r; {
                (*(*j).d).u = (*j).u;
                (*(*j).u).d = (*j).d;
                (*(*j).c).n.1 -= 1;
            })
        })
    }

    unsafe fn uncover(col: *mut Node) {
        cfor!(let mut i = (*col).u; i != col; i = (*i).u; {
            cfor!(let mut j = (*i).l; j != i; j = (*j).l; {
                (*(*j).d).u = j;
                (*(*j).u).d = j;
                (*(*j).c).n.1 += 1;
            })
        });

        (*(*col).r).l = col;
        (*(*col).l).r = col
    }

    unsafe fn algorithm_x(root: *mut Node, nresult: &mut usize) {
        let mut c = root;

        cfor!(let mut x = (*c).r; x != root; x = (*x).r; {
            if (*x).n.0 == 1 && (*x).n.1 < (*c).n.1 {
                c = x
            }
        });

        if c == root {
            return *nresult += 1
        }

        cover(c);
        cfor!(let mut r = (*c).d; r != c; r = (*r).d; {
            cfor!(let mut j = (*r).r; r != j; j = (*j).r; {
                cover((*j).c)
            });

            algorithm_x(root, nresult);

            cfor!(let mut j = (*r).l; r != j; j = (*j).l; {
                uncover((*j).c)
            })
        });
        uncover(c);
    }

    #[allow(non_snake_case)]
    unsafe fn add_row(columns: &[*mut Node], name: (usize, usize)) {
        let mut nodes: Box<[_]> = columns.iter().map(|&c| {
            let mut node = Node::new();
            (*node).n = name;
            (*node).u = (*c).u;
            (*node).d = c;
            (*node).c = c;
            (*(*c).u).d = node;
            (*c).u = node;
            (*c).n.1 += 1;
            node
        }).collect();

        let N = nodes.len();
        for i in 0..N {
            (*nodes[i]).r = nodes[(i+1) % N];
            (*nodes[i]).l = nodes[(i+N-1) % N];
        }
    }

    // first n are horizonal constraints,
    // next n are verticle constraints,
    // next 2n-1 are slash(/) constraints,
    // and last 2n-1 are backslash(\) constraints
    let n = n as usize;
    let ncol = 6*n-2;
    let mut columns: Vec<_> = (0..ncol).map(|_| Node::new()).collect();
    for i in 0..ncol {
        (*columns[i]).u = columns[i];
        (*columns[i]).d = columns[i];
        (*columns[i]).c = columns[i];
        (*columns[i]).r = columns[(i+1) % ncol];
        (*columns[i]).l = columns[(i+ncol-1) % ncol];
    }
    for i in 0..2*n { // must be covered
        (*columns[i]).n.0 = 1
    }

    // rows. The i*n+j element means put a queen at (i, j).
    for i in 0..n {
        for j in 0..n {
            add_row(&[
                columns[i], // i-th row has a queen
                columns[n + j], // j-th col has a queen
                columns[2*n + i + j], // slash line
                columns[4*n-1 + i + n-1-j] // backslash line
            ], (i, j));
        }
    }

    let root = Node::new();
    (*root).l = *columns.last().unwrap();
    (*root).r = *columns.first().unwrap();
    (*root).n.1 = std::usize::MAX;
    (**columns.first().unwrap()).l = root;
    (**columns.last().unwrap()).r = root;

    let mut nresult = 0;
    algorithm_x(root, &mut nresult);
    nresult as _
}

fn main() {
    assert_eq!(unsafe { total_n_queens(4) }, 2);
}

// faster than 100.00%, less memory than 100.00%.
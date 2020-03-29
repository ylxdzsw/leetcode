// Write a program to solve a Sudoku puzzle by filling the empty cells.

// A sudoku solution must satisfy all of the following rules:

// Each of the digits 1-9 must occur exactly once in each row.
// Each of the digits 1-9 must occur exactly once in each column.
// Each of the the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
// Empty cells are indicated by the character '.'.

// Note:

// The given board contain only digits 1-9 and the character '.'.
// You may assume that the given Sudoku puzzle will have a single unique solution.
// The given board size is always 9x9.

struct Node {
    pub n: (usize, usize, usize), // (i, j, number). For normal node, this is used for printing the answer. For header (column) nodes, only number is used, which indicates how many nodes is linked to it vertically
    pub l: *mut Node,
    pub r: *mut Node,
    pub u: *mut Node,
    pub d: *mut Node,
    pub c: *mut Node
}

#[allow(clippy::zero_ptr)]
impl Node {
    pub fn new() -> *mut Node {
        let node = Box::new(Node { n: (0, 0, 0), l: 0 as *mut Node, r: 0 as *mut Node, u: 0 as *mut Node, d: 0 as *mut Node, c: 0 as *mut Node });
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

unsafe fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    unsafe fn cover(col: *mut Node) { // col must be a column (header) node
        (*(*col).r).l = (*col).l;
        (*(*col).l).r = (*col).r;

        cfor!(let mut i = (*col).d; i != col; i = (*i).d; {
            cfor!(let mut j = (*i).r; j != i; j = (*j).r; {
                (*(*j).d).u = (*j).u;
                (*(*j).u).d = (*j).d;
                (*(*j).c).n.2 -= 1;
            })
        })
    }

    unsafe fn uncover(col: *mut Node) {
        cfor!(let mut i = (*col).u; i != col; i = (*i).u; {
            cfor!(let mut j = (*i).l; j != i; j = (*j).l; {
                (*(*j).d).u = j;
                (*(*j).u).d = j;
                (*(*j).c).n.2 += 1;
            })
        });

        (*(*col).r).l = col;
        (*(*col).l).r = col
    }

    unsafe fn algorithm_x(root: *mut Node) -> Option<Vec<(usize, usize, usize)>> {
        let mut c = (*root).r;
        if c == root {
            return Some(vec![])
        }

        cfor!(let mut x = (*c).r; x != root; x = (*x).r; {
            if (*x).n.2 < (*c).n.2 {
                c = x
            }
        });

        cover(c);
        cfor!(let mut r = (*c).d; r != c; r = (*r).d; {
            cfor!(let mut j = (*r).r; r != j; j = (*j).r; {
                cover((*j).c)
            });

            if let Some(mut ans) = algorithm_x(root) {
                ans.push((*r).n);
                return Some(ans)
            }

            cfor!(let mut j = (*r).l; r != j; j = (*j).l; {
                uncover((*j).c)
            })
        });
        uncover(c);

        None
    }

    #[allow(non_snake_case)]
    unsafe fn add_row(columns: &[*mut Node], name: (usize, usize, usize)) {
        let mut nodes: Box<[_]> = columns.iter().map(|&c| {
            let mut node = Node::new();
            (*node).n = name;
            (*node).u = (*c).u;
            (*node).d = c;
            (*node).c = c;
            (*(*c).u).d = node;
            (*c).u = node;
            (*c).n.2 += 1;
            node
        }).collect();

        let N = nodes.len();
        for i in 0..N {
            (*nodes[i]).r = nodes[(i+1) % N];
            (*nodes[i]).l = nodes[(i+N-1) % N];
        }
    }

    // first 9*9 are horizonal constraints,
    // next 9*9 are verticle constraints,
    // next 9*9 are block constraints,
    // and last 9*9 constraints restrct that only one number in a cell
    let ncol = 9 * 9 * 4;
    let mut columns: Vec<_> = (0..ncol).map(|_| Node::new()).collect();
    for i in 0..ncol {
        (*columns[i]).u = columns[i];
        (*columns[i]).d = columns[i];
        (*columns[i]).c = columns[i];
        (*columns[i]).r = columns[(i+1) % ncol];
        (*columns[i]).l = columns[(i+ncol-1) % ncol];
    }

    // rows. The i*9*9+j*9+k element means if the number in (i, j) is k+1.
    for i in 0..9 {
        for j in 0..9 {
            let v = board[i][j].to_digit(10).map(|x| x-1);
            for k in 0..9 {
                if v.is_some() && v.unwrap() as usize != k {
                    continue // this row cannot be selected because it is already fixed on the board
                }
                add_row(&[
                    columns[i*9 + k], // i-th row has k
                    columns[9*9 + j*9 + k], // j-th col has k
                    columns[2*9*9 + (3*(i/3)+j/3)*9 + k], // the block has k
                    columns[3*9*9 + i*9 + j] // (i, j) is occupied
                ], (i, j, k));
            }
        }
    }

    let root = Node::new();
    (*root).l = *columns.last().unwrap();
    (*root).r = *columns.first().unwrap();
    (**columns.first().unwrap()).l = root;
    (**columns.last().unwrap()).r = root;

    for (i, j, k) in algorithm_x(root).unwrap() {
        board[i][j] = std::char::from_digit((k+1) as _, 10).unwrap()
    }
}

fn main() {
    let mut problem = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let answer = vec![
        vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];
    unsafe { solve_sudoku(&mut problem) };
    assert_eq!(problem, answer);
}

// faster than 100.00%, less memory than 100.00%.
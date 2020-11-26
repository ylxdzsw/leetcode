// Write an API that generates fancy sequences using the append, addAll, and multAll operations.

// Implement the Fancy class:

// Fancy() Initializes the object with an empty sequence.
// void append(val) Appends an integer val to the end of the sequence.
// void addAll(inc) Increments all existing values in the sequence by an integer inc.
// void multAll(m) Multiplies all existing values in the sequence by an integer m.
// int getIndex(idx) Gets the current value at index idx (0-indexed) of the sequence modulo 109 + 7. If the index is greater or equal than the length of the sequence, return -1.

// Example 1:

// Input
// ["Fancy", "append", "addAll", "append", "multAll", "getIndex", "addAll", "append", "multAll", "getIndex", "getIndex", "getIndex"]
// [[], [2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]]
// Output
// [null, null, null, null, null, 10, null, null, null, 26, 34, 20]

// Explanation
// Fancy fancy = new Fancy();
// fancy.append(2);   // fancy sequence: [2]
// fancy.addAll(3);   // fancy sequence: [2+3] -> [5]
// fancy.append(7);   // fancy sequence: [5, 7]
// fancy.multAll(2);  // fancy sequence: [5*2, 7*2] -> [10, 14]
// fancy.getIndex(0); // return 10
// fancy.addAll(3);   // fancy sequence: [10+3, 14+3] -> [13, 17]
// fancy.append(10);  // fancy sequence: [13, 17, 10]
// fancy.multAll(2);  // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]
// fancy.getIndex(0); // return 26
// fancy.getIndex(1); // return 34
// fancy.getIndex(2); // return 20

// Constraints:

// 1 <= val, inc, m <= 100
// 0 <= idx <= 105
// At most 105 calls total will be made to append, addAll, multAll, and getIndex.

// copied from https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

const MODULO: u64 = 1_000_000_007;

struct Fancy {
    raw: Vec<u64>,
    mult_acc: u64,
    add_acc: u64,
}

impl Fancy {
    fn new() -> Self {
        Self { raw: Vec::with_capacity(10000), mult_acc: 1, add_acc: 0 }
    }

    // the key part: by Fermat's theorem, a^(m-1) ≡ 1 (mod m)
    // multiply both sides with a^(-1) and we have a^(-1) ≡ a^(m-2) (mod m)
    // therefore we can adjust the coming number such that it will recover by the accumulate formula.
    fn append(&mut self, val: i32) {
        let mut val = val as u64;
        val += MODULO - self.add_acc; // in case of it is smaller
        val %= MODULO;
        val *= mod_pow(self.mult_acc, MODULO - 2, MODULO);
        val %= MODULO;
        self.raw.push(val)
    }

    fn add_all(&mut self, inc: i32) {
        self.add_acc += inc as u64;
        self.add_acc %= MODULO;
    }

    fn mult_all(&mut self, m: i32) {
        self.mult_acc *= m as u64;
        self.mult_acc %= MODULO;
        self.add_acc *= m as u64;
        self.add_acc %= MODULO;
    }

    fn get_index(&mut self, idx: i32) -> i32 {
        if let Some(x) = self.raw.get(idx as usize) {
            let mut x = *x;
            x *= self.mult_acc;
            x %= MODULO;
            x += self.add_acc;
            x %= MODULO;
            x as _
        } else {
            -1
        }
    }
}

fn main() {
    let mut fancy = Fancy::new();
    fancy.append(2);   // fancy sequence: [2]
    fancy.add_all(3);   // fancy sequence: [2+3] -> [5]
    fancy.append(7);   // fancy sequence: [5, 7]
    fancy.mult_all(2);  // fancy sequence: [5*2, 7*2] -> [10, 14]
    assert_eq!(fancy.get_index(0), 10);
    fancy.add_all(3);   // fancy sequence: [10+3, 14+3] -> [13, 17]
    fancy.append(10);  // fancy sequence: [13, 17, 10]
    fancy.mult_all(2);  // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]
    assert_eq!(fancy.get_index(0), 26);
    assert_eq!(fancy.get_index(1), 34);
    assert_eq!(fancy.get_index(2), 20);
}

// faster than 58.33%, less memory than 25.00%.
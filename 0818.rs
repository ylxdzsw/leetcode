// Your car starts at position 0 and speed +1 on an infinite number line.  (Your car can go into negative positions.)

// Your car drives automatically according to a sequence of instructions A (accelerate) and R (reverse).

// When you get an instruction "A", your car does the following: position += speed, speed *= 2.

// When you get an instruction "R", your car does the following: if your speed is positive then speed = -1 , otherwise speed = 1.  (Your position stays the same.)

// For example, after commands "AAR", your car goes to positions 0->1->3->3, and your speed goes to 1->2->4->-1.

// Now for some target position, say the length of the shortest sequence of instructions to get there.

// Example 1:
// Input: 
// target = 3
// Output: 2
// Explanation: 
// The shortest instruction sequence is "AA".
// Your position goes from 0->1->3.

// Example 2:
// Input: 
// target = 6
// Output: 5
// Explanation: 
// The shortest instruction sequence is "AAARA".
// Your position goes from 0->1->3->7->7->6.
 
// Note:

// 1 <= target <= 10000.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    target: i32, // Assume target is positive. If it is negative, we inverse both target and speed.
    speed: i32,
    length: i32 // The number of actions needed to reach this State.
}

impl State {
    fn score(&self) -> i32 {
        -self.length // simple BFS. note that `target` is not a proper heuristic for A*. Not sure about `log(target)`.
    }
}

impl core::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl core::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<core::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn racecar(target: i32) -> i32 {
    let mut cache: std::collections::BTreeMap<(i32, i32), i32> = Default::default(); // (target, speed) to length
    let mut queue: std::collections::BinaryHeap<State> = std::collections::BinaryHeap::with_capacity(10000); // the states to explore next

    cache.insert((target, 1), 0);
    queue.push(State { target, speed: 1, length: 0 });

    fn explore(mut target: i32, mut speed: i32, length: i32, cache: &mut std::collections::BTreeMap<(i32, i32), i32>, queue: &mut std::collections::BinaryHeap<State>) {
        if target < 0 { // ensure target >= 0
            target = -target;
            speed = -speed;
        }

        if let Some(l) = cache.get_mut(&(target, speed)) {
            if *l <= length {// we have already explored a non-worse path
                // do nothing
            } else {
                *l = length;
                queue.push(State { target, speed, length})    
            }
        } else {
            cache.insert((target, speed), length);
            queue.push(State { target, speed, length})
        };
    };

    while let Some(state) = queue.pop() {
        let State { target, speed, length } = state.clone();

        if target == 0 {
            return length
        }

        explore(target - speed, speed * 2, length + 1, &mut cache, &mut queue);
        explore(target, -speed.signum(), length + 1, &mut cache, &mut queue);
    }

    unreachable!()
}

fn main() {
    assert_eq!(racecar(3), 2);
    assert_eq!(racecar(6), 5);
}

// faster than 100.00%, less memory than 100.00%.
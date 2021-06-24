// You are given n​​​​​​ tasks labeled from 0 to n - 1 represented by a 2D integer array tasks, where tasks[i] = [enqueueTimei, processingTimei] means that the i​​​​​​th​​​​ task will be available to process at enqueueTimei and will take processingTimei to finish processing.

// You have a single-threaded CPU that can process at most one task at a time and will act in the following way:

// If the CPU is idle and there are no available tasks to process, the CPU remains idle.
// If the CPU is idle and there are available tasks, the CPU will choose the one with the shortest processing time. If multiple tasks have the same shortest processing time, it will choose the task with the smallest index.
// Once a task is started, the CPU will process the entire task without stopping.
// The CPU can finish a task then start a new one instantly.
// Return the order in which the CPU will process the tasks.

// Example 1:

// Input: tasks = [[1,2],[2,4],[3,2],[4,1]]
// Output: [0,2,3,1]
// Explanation: The events go as follows: 
// - At time = 1, task 0 is available to process. Available tasks = {0}.
// - Also at time = 1, the idle CPU starts processing task 0. Available tasks = {}.
// - At time = 2, task 1 is available to process. Available tasks = {1}.
// - At time = 3, task 2 is available to process. Available tasks = {1, 2}.
// - Also at time = 3, the CPU finishes task 0 and starts processing task 2 as it is the shortest. Available tasks = {1}.
// - At time = 4, task 3 is available to process. Available tasks = {1, 3}.
// - At time = 5, the CPU finishes task 2 and starts processing task 3 as it is the shortest. Available tasks = {1}.
// - At time = 6, the CPU finishes task 3 and starts processing task 1. Available tasks = {}.
// - At time = 10, the CPU finishes task 1 and becomes idle.

// Example 2:

// Input: tasks = [[7,10],[7,12],[7,5],[7,4],[7,2]]
// Output: [4,3,2,0,1]
// Explanation: The events go as follows:
// - At time = 7, all the tasks become available. Available tasks = {0,1,2,3,4}.
// - Also at time = 7, the idle CPU starts processing task 4. Available tasks = {0,1,2,3}.
// - At time = 9, the CPU finishes task 4 and starts processing task 3. Available tasks = {0,1,2}.
// - At time = 13, the CPU finishes task 3 and starts processing task 2. Available tasks = {0,1}.
// - At time = 18, the CPU finishes task 2 and starts processing task 0. Available tasks = {1}.
// - At time = 28, the CPU finishes task 0 and starts processing task 1. Available tasks = {}.
// - At time = 40, the CPU finishes task 1 and becomes idle.
 
// Constraints:

// tasks.length == n
// 1 <= n <= 105
// 1 <= enqueueTimei, processingTimei <= 109

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Task {
    id: usize,
    time: i32,
}

impl core::cmp::Ord for Task {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.time.cmp(&other.time).then(self.id.cmp(&other.id)).reverse()
    }
}

impl core::cmp::PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let mut incoming: std::collections::BinaryHeap<_> = tasks.iter().enumerate().map(|(id, times)| Task { id, time: times[0] }).collect();
    let mut avaliable: std::collections::BinaryHeap<Task> = std::collections::BinaryHeap::with_capacity(tasks.len());

    let mut record: Vec<i32> = vec![];
    let mut cpu_time = 0;

    loop {
        // update avaliable tasks
        while let Some(true) = incoming.peek().map(|x| x.time <= cpu_time) {
            let id = incoming.pop().unwrap().id;
            avaliable.push(Task { id, time: tasks[id][1] });
        }

        // execute one
        if let Some(task) = avaliable.pop() {
            cpu_time += task.time;
            record.push(task.id as _)
        } else { // idle or done
            if incoming.is_empty() { // done
                return record
            } else { // idle
                cpu_time = incoming.peek().unwrap().time
            }
        }
    }
}

fn main() {
    assert_eq!(get_order(vec![vec![1,2],vec![2,4],vec![3,2],vec![4,1]]), [0,2,3,1]);
    assert_eq!(get_order(vec![vec![7,10],vec![7,12],vec![7,5],vec![7,4],vec![7,2]]), [4,3,2,0,1])
}

// faster than 16.67%, less memory than 50%.
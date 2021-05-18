pub struct Solution;

impl Solution {
    // prerequisites: Vec<(i32, 32)> 0 <- 1
    // O(m + n)
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        // for a given course number, the number of requirements it has
        let mut num_requirements = vec![0; num_courses];
        // for a given course number, all courses that depend on it
        let mut children = vec![Vec::with_capacity(num_courses); num_courses];
        // topologically ordered course numbers
        let mut result = Vec::with_capacity(num_courses);

        // for each subgraph r[0] -> r[1], contained in prerequisites, that represent one
        // individual dependency relationship in the overall digraph.
        prerequisites.iter().for_each(|req| {
            // we record that dependency
            children[req[1] as usize].push(req[0]);
            // and record the total number of dependencies for that course / digraph node.
            num_requirements[req[0] as usize] += 1;
        });

        // Our BFS Stack
        let mut stack = vec![];

        // we push all the sink nodes into our stack, as we want to start BFS from them.
        for i in 0..num_courses {
            if num_requirements[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(course) = stack.pop() {
            // if this is a sink node, push it to our topo-sorted node list.
            result.push(course as i32);
            // now look at all the nodes that point to this sink
            for n in &children[course] {
                let n = *n as usize;
                // it has one less requirement, as the sink node in question is now being "pruned"
                // from the digraph.
                num_requirements[n] -= 1;
                // if at this point it has no more requirements, it is now a sink node, and we want
                // to consider it in our search.
                if num_requirements[n] == 0 {
                    stack.push(n);
                }
            }
        }

        // at the end, we expect to have looked at all the nodes in the digraph, and constructed a
        // total ordering of them. If this isn't the case, it must mean the graph is cyclic or
        // clustered, and we return an empty result.
        if result.len() == num_courses {
            result
        } else {
            Vec::new()
        }
    }
}

/*
O(m + n)
L ← Empty list that will contain the sorted nodes
while exists nodes without a permanent mark do
    select an unmarked node n
    visit(n)

function visit(node n)
    if n has a permanent mark then
        return
    if n has a temporary mark then
        stop   (not a DAG)

    mark n with a temporary mark

    for each node m with an edge from n to m do
        visit(m)

    remove temporary mark from n
    mark n with a permanent mark
    add n to head of L
*/

fn main() {
    println!("Hello, world!");
}

# Incorrect solution

This is an initial solution I came up with which is actually wrong for precisely the reason I 
guessed in the initial commit.

The proof of the route from my input is in the [Visualitzation of the final route](path_for_input.visualization).

Documenting the wrong approach for historical context.

```
/*
**********************************
Initial approach of doing a sweep.
**********************************
This works for the test, but not the actual input. Most likely there is an upwards or leftward
traversal that I should account for in the main input. It's easy to construct an example
that forces the route around a bunch of 9's using all 1's.

Funnily, this gives an "off by one" error to the answer. Need to then find the bug in it.
Shouldn't be a coincidence. Maybe then there's no upwards or leftwards traversal.
*/
#[allow(dead_code)]
fn solve_attempt_one(input: &Vec<Vec<u32>>, limit: usize) {
    let mut costs = vec![vec![(u32::MAX, (0, 0)); limit + 1]; limit + 1];
    costs[0][0].0 = 0;
    (0..=limit).for_each(|row| {
        (0..=limit).for_each(|col| {
            if row < limit {
                let pot_cost_1 = costs[row][col].0 + input[row + 1][col];
                if costs[row + 1][col].0 > pot_cost_1 {
                    costs[row + 1][col] = (pot_cost_1, (row, col));
                }
            }
            if col < limit {
                let pot_cost_2 = costs[row][col].0 + input[row][col + 1];
                if costs[row][col + 1].0 > pot_cost_2 {
                    costs[row][col + 1] = (pot_cost_2, (row, col));
                }
            }
        })
    });
    costs.iter().for_each(|row| println!("{:?}", row));
    println!("{:?}", costs[limit][limit]);
}

```
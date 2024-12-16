#![feature(gen_blocks, let_chains)]

use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc2024::aoc_solution;
use itertools::{Itertools, iproduct};
use pathfinding::prelude::dijkstra;

fn main() -> eyre::Result<()> {
    aoc_solution(16, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let (start_x, start_y) = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'S')
        .unwrap();

    let end = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'E')
        .unwrap();

    let (_, cost) = dijkstra(
        &(start_x, start_y, 1, 0),
        |&state| successors(&grid, state),
        |&(x, y, _, _)| (x, y) == end,
    )
    .unwrap();

    println!("{cost}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let (start_x, start_y) = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'S')
        .unwrap();

    let (end_x, end_y) = iproduct!(0..width, 0..height)
        .find(|&(x, y)| grid[y][x] == 'E')
        .unwrap();

    let mut dists = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut parents = HashMap::<_, HashSet<_>>::new();

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct State((usize, usize, isize, isize), usize);

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    dists.insert((start_x, start_y, 1, 0), 0);
    heap.push(State((start_x, start_y, 1, 0), 0));

    // Run a modified version of Dijkstra that stores all parents for a node with the same minimum distance
    // instead of just one.
    while let Some(State(a, a_dist)) = heap.pop() {
        for (b, c) in successors(&grid, a) {
            let b_dist = *dists.get(&b).unwrap_or(&usize::MAX);
            let new_b_dist = a_dist + c;
            // We relax the usual strict '<' condition to '<=', to handle cases where there are multiple
            // paths with the same distance.
            if new_b_dist <= b_dist {
                let b_parents = parents.entry(b).or_default();

                // If the new distance *is* strictly shorter, clear all existing parents (which must be
                // via a longer route) and update the distance.
                if new_b_dist < b_dist {
                    b_parents.clear();
                    dists.insert(b, new_b_dist);
                }

                // Add a to b's parents set.
                b_parents.insert(a);

                heap.push(State(b, new_b_dist));
            }
        }
    }

    // Find the minimum distance to the end tile, and all possible end states (i.e. directions)
    // that satisfy this.
    let mut min_dist = usize::MAX;
    let mut end_states = vec![];
    for (&(x, y, dx, dy), &d) in &dists {
        if (x, y) != (end_x, end_y) || d > min_dist {
            continue;
        }

        if d < min_dist {
            end_states.clear();
            min_dist = d;
        }

        end_states.push((x, y, dx, dy));
    }

    // Collect all states on any given shortest path to the end states.
    let mut states = HashSet::new();
    let mut stack = end_states;
    while let Some((x, y, dx, dy)) = stack.pop() {
        if !states.insert((x, y, dx, dy)) {
            continue;
        }

        if let Some(parents) = parents.get(&(x, y, dx, dy)) {
            stack.extend(parents);
        }
    }

    // Finally, discard the dx and dy values from the above states to find just the set of
    // tile positions on the shortest paths.
    let tiles = states
        .into_iter()
        .map(|(x, y, _, _)| (x, y))
        .collect::<HashSet<_>>();

    println!("{}", tiles.len());

    Ok(())
}

type Grid = Vec<Vec<char>>;

fn successors(
    grid: &Grid,
    state: (usize, usize, isize, isize),
) -> impl Iterator<Item = ((usize, usize, isize, isize), usize)> {
    let (x, y, dx, dy) = state;

    let width = grid[0].len();
    let height = grid.len();

    gen move {
        if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy))
            && x < width
            && y < height
            && grid[y][x] != '#'
        {
            yield ((x, y, dx, dy), 1);
        }

        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if dir != (dx, dy) {
                yield ((x, y, dir.0, dir.1), 1000);
            }
        }
    }
}

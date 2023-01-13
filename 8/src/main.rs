use std::cmp::max;

use ndarray::prelude::*;
use ndarray::{Array, Dim};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Tree(u8);

type Dim2D = Dim<[usize; 2]>;

fn main() {
    
    let input: Vec<&str> = include_str!("../input.txt").lines().collect();

    let trees = read_into_matrix(input);

    let visible_trees = calculate_visible_trees(&trees);

    println!("Visible trees: {visible_trees}");

}

fn read_into_matrix(input: Vec<&str>) -> Array<Tree, Dim2D> {

    let nb_rows = input[0].len();
    let nb_cols = input.len();

    Array::from_iter(
        input.join("").bytes().map(|b| Tree(b - b'0'))
    ).into_shape((nb_rows, nb_cols)).unwrap()

}

fn calculate_visible_trees(tree_map: &Array<Tree, Dim2D>) -> usize {

    let nb_rows = tree_map.len_of(Axis(0));
    let nb_cols = tree_map.len_of(Axis(1));

    // Minus 2 because the corners would be counted twice
    let mut visible_trees = (nb_rows + nb_cols - 2) * 2;

    // 1..x - 1 in order to skip the corners, which are always visible and thus
    // were accounted for in the initial value of visible_trees
    for ((row_idx, col_idx), tree) in tree_map.slice(s![1..nb_rows - 1, 1..nb_cols - 1]).indexed_iter() {
        
        let row_idx = row_idx + 1;
        let col_idx = col_idx + 1;

        let row = tree_map.slice(s![row_idx, ..]);
        let col = tree_map.slice(s![.., col_idx]);

        let compare_trees = |tallest_tree_height: u8, tree: &Tree| max(tallest_tree_height, tree.0);

        let tallest_tree_row_left = row.slice(s![..col_idx]).iter().fold(0, compare_trees);
        let tallest_tree_row_right = row.slice(s![col_idx + 1..]).iter().fold(0, compare_trees);

        let tallest_tree_col_up = col.slice(s![..row_idx]).iter().fold(0, compare_trees);
        let tallest_tree_col_down = col.slice(s![row_idx + 1..]).iter().fold(0, compare_trees);

        let visible_trees = &mut visible_trees;

        [
            tallest_tree_row_left,
            tallest_tree_row_right,
            tallest_tree_col_up,
            tallest_tree_col_down,
        ].iter().any(|t| t < &tree.0).then(|| *visible_trees += 1);

    }

    visible_trees

}
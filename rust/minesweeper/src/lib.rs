pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Iterate through the rows of the minefield
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            // Iterate through the cells in the row
            row.chars()
                .enumerate()
                .map(|(j, cell)| {
                    // If the cell is a mine, add it to the annotated minefield as is
                    if cell == '*' {
                        '*'
                    } else {
                        // Otherwise, count the number of mines surrounding the cell
                        let count = count_adjacent_mines(minefield, i, j);
                        // If there are no mines surrounding the cell, add a blank space to the annotated minefield
                        // Otherwise, add the count of mines to the annotated minefield
                        if count == 0 {
                            ' '
                        } else {
                            count.to_string().chars().next().unwrap()
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn count_adjacent_mines(minefield: &[&str], i: usize, j: usize) -> usize {
    // Get the number of rows and columns in the minefield
    let rows = minefield.len();
    let cols = minefield[0].len();

    // Iterate through the rows and columns surrounding the given square (i, j)
    (i.saturating_sub(1)..=i + 1)
        .flat_map(|row| (j.saturating_sub(1)..=j + 1).map(move |col| (row, col)))
        // Filter out the squares that are not adjacent to (i, j) or are out of bounds
        .filter(|(row, col)| {
            (row != &i || col != &j)
                && row < &rows
                && col < &cols
                && &minefield[*row][*col..*col + 1] == "*"
        })
        // Map the remaining squares to 1, representing the fact that they contain a mine
        .map(|_| 1)
        // Sum the mapped values to get the total number of mines
        .sum()
}

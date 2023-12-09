use std::fs;

#[derive(Debug)]
struct BoundingBox {
    top_left: Option<(usize, usize)>,
    top: Option<(usize, usize)>,
    top_right: Option<(usize, usize)>,
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
    bottom_left: Option<(usize, usize)>,
    bottom: Option<(usize, usize)>,
    bottom_right: Option<(usize, usize)>,
}

struct BoundingBoxIterator<'a> {
    bounding_box: &'a BoundingBox,
    index: usize,
}

impl<'a> Iterator for BoundingBoxIterator<'a> {
    type Item = &'a Option<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(&self.bounding_box.top_left),
            1 => Some(&self.bounding_box.top),
            2 => Some(&self.bounding_box.top_right),
            3 => Some(&self.bounding_box.left),
            4 => Some(&self.bounding_box.right),
            5 => Some(&self.bounding_box.bottom_left),
            6 => Some(&self.bounding_box.bottom),
            7 => Some(&self.bounding_box.bottom_right),
            _ => None,
        };
        self.index += 1;
        result
    }
}

impl BoundingBox {
    pub fn iter(&self) -> BoundingBoxIterator {
        BoundingBoxIterator {
            bounding_box: self,
            index: 0,
        }
    }
}

fn get_bounding_box(matrix: &Vec<Vec<char>>, coordinates: (usize, usize)) -> BoundingBox {
    let (row, col) = coordinates;
    let num_of_rows = matrix.len();
    let row_len = matrix.get(0).unwrap().len();

    BoundingBox {
        top_left: matrix
            .get(row.checked_sub(1).unwrap_or(num_of_rows))
            .and_then(|line| {
                let ch = line.get(col.checked_sub(1).unwrap_or(row_len));
                if let Some(character) = ch {
                    if character.is_numeric() {
                        return Some((row - 1, col - 1));
                    }
                }
                None
            }),
        top: matrix
            .get(row.checked_sub(1).unwrap_or(num_of_rows))
            .and_then(|line| {
                let ch = line.get(col);
                if let Some(character) = ch {
                    if character.is_numeric() {
                        return Some((row - 1, col));
                    }
                }
                None
            }),
        top_right: matrix
            .get(row.checked_sub(1).unwrap_or(num_of_rows))
            .and_then(|line| {
                let ch = line.get(col + 1);
                if let Some(character) = ch {
                    if character.is_numeric() {
                        return Some((row - 1, col + 1));
                    }
                }
                None
            }),
        left: matrix.get(row).and_then(|line| {
            let ch = line.get(col.checked_sub(1).unwrap_or(row_len));
            if let Some(character) = ch {
                if character.is_numeric() {
                    return Some((row, col - 1));
                }
            }
            None
        }),
        right: matrix.get(row).and_then(|line| {
            let ch = line.get(col + 1);
            if let Some(character) = ch {
                if character.is_numeric() {
                    return Some((row, col + 1));
                }
            }
            None
        }),
        bottom_left: matrix.get(row + 1).and_then(|line| {
            let ch = line.get(col.checked_sub(1).unwrap_or(row_len));
            if let Some(character) = ch {
                if character.is_numeric() {
                    return Some((row + 1, col - 1));
                }
            }
            None
        }),
        bottom: matrix.get(row + 1).and_then(|line| {
            let ch = line.get(col);
            if let Some(character) = ch {
                if character.is_numeric() {
                    return Some((row + 1, col));
                }
            }
            None
        }),
        bottom_right: matrix.get(row + 1).and_then(|line| {
            let ch = line.get(col + 1);
            if let Some(character) = ch {
                if character.is_numeric() {
                    return Some((row + 1, col + 1));
                }
            }
            None
        }),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input_lines = input.split('\n').collect::<Vec<_>>();

    let lines: Vec<Vec<char>> = input_lines
        .iter()
        .map(|&line| line.chars().collect::<Vec<char>>())
        .collect();

    let symbol_indices = lines
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, &ch)| ch != '.' && !ch.is_numeric())
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let bounding_boxes = symbol_indices
        .iter()
        .enumerate()
        .map(|(row, indices)| {
            indices
                .iter()
                .map(|&index| get_bounding_box(&lines, (row, index)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut nums: Vec<Vec<u32>> = vec![];

    bounding_boxes.into_iter().for_each(|bounds| {
        bounds.iter().for_each(|b_box| {
            let valid_coords = b_box
                .iter()
                .filter(|&coord| coord.is_some())
                .map(|coord| coord.unwrap())
                .collect::<Vec<_>>();

            let numbers = valid_coords
                .iter()
                .map(|&(row, col)| {
                    let (back_range, forward_range) = (0..col, col..);
                    let current_row = lines.get(row).unwrap();

                    let back = &current_row[back_range]
                        .into_iter()
                        .rev()
                        .take_while(|&ch| ch.is_numeric())
                        .map(|ch| *ch)
                        .collect::<Vec<_>>();

                    let front = &current_row[forward_range]
                        .into_iter()
                        .take_while(|&ch| ch.is_numeric())
                        .map(|ch| *ch)
                        .collect::<Vec<_>>();

                    let num = back
                        .into_iter()
                        .rev()
                        .chain(front.into_iter())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();

                    num
                })
                .collect::<Vec<_>>();

            nums.push(numbers);
        })
    });

    nums.iter_mut().for_each(|ele| {
        ele.sort();
        ele.dedup();
    });

    println!(
        "{:?}",
        nums.iter()
            .map(|ele| { ele.iter().sum::<u32>() })
            .sum::<u32>()
    );

    let real_parts = nums
        .iter()
        .filter(|&n| n.len() == 2)
        .map(|n| n.iter().fold(1u32, |acc, &curr| acc * curr))
        .sum::<u32>();

    println!("{}", real_parts);
}

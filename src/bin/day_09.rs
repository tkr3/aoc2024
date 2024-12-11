use std::collections::VecDeque;

pub fn part1(input: &str) -> u64 {
    let blocks: VecDeque<_> = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit") as u64)
        .enumerate()
        .map(|(index, len)| {
            if index % 2 == 0 {
                DiskNode::File(len, index as u64 / 2)
            } else {
                DiskNode::Free(len)
            }
        })
        .collect();

    DiskFrag::new(blocks)
        .enumerate()
        .map(|(a, b)| a as u64 * b)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut blocks: VecDeque<_> = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit") as u64)
        .enumerate()
        .map(|(index, len)| {
            if index % 2 == 0 {
                DiskNode::File(len, index as u64 / 2)
            } else {
                DiskNode::Free(len)
            }
        })
        .collect();

    let mut next_id = blocks.len() / 2;

    loop {
        let mut next_index: Option<usize> = None;
        let mut next_block: Option<DiskNode> = None;
        for (index, block) in blocks.iter().enumerate().rev() {
            if let DiskNode::File(_size, file_id) = block {
                if *file_id == next_id as u64 {
                    next_block = Some(block.clone());
                    next_index = Some(index);
                }
            }
        }

        if let next @ DiskNode::File(file_size, file_id) = next_block.unwrap() {
            let mut next_free: Option<DiskNode> = None;
            let mut next_free_index: Option<usize> = None;
            for (index, block) in blocks.iter().enumerate() {
                match block {
                    DiskNode::File(_, id) if *id == file_id => {
                        break;
                    }
                    free @ DiskNode::Free(size) if *size >= file_size => {
                        next_free = Some(free.clone());
                        next_free_index = Some(index);
                        break;
                    }
                    _ => {}
                }
            }

            if let Some(DiskNode::Free(free)) = next_free {
                blocks[next_index.unwrap()] = DiskNode::Free(file_size);
                blocks[next_free_index.unwrap()] = next;
                if file_size < free {
                    blocks.insert(
                        next_free_index.unwrap() + 1,
                        DiskNode::Free(free - file_size),
                    )
                }
            }
        } else {
            unreachable!("DiskNode should be file")
        }

        if let Some(next) = next_id.checked_sub(1) {
            next_id = next;
        } else {
            break;
        }
    }

    blocks
        .into_iter()
        .fold((0, 0), |(index, sum), block| match block {
            DiskNode::Free(size) => (index + size, sum),
            DiskNode::File(size, id) => (
                index + size,
                (index..index + size).map(|i| id * i).sum::<u64>() + sum,
            ),
        })
        .1
}

#[derive(Debug, Clone)]
enum DiskNode {
    File(u64, u64),
    Free(u64),
}

#[derive(Debug, Default)]
struct DiskFrag {
    blocks: VecDeque<DiskNode>,
    current_block: Option<DiskNode>,
    insert_block: Option<DiskNode>,
}

impl DiskFrag {
    fn new(blocks: VecDeque<DiskNode>) -> Self {
        DiskFrag {
            blocks,
            ..Default::default()
        }
    }
}

impl Iterator for DiskFrag {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = &self.current_block.take().or_else(|| loop {
            let next = self.blocks.pop_front();
            if matches!(next, Some(DiskNode::Free(0))) {
                continue;
            }
            return next;
        }) {
            match node {
                DiskNode::File(size, id) => {
                    if *size > 1 {
                        let _ = self.current_block.insert(DiskNode::File(size - 1, *id));
                    }
                    return Some(*id);
                }
                DiskNode::Free(size) => {
                    debug_assert!(*size >= 1);
                    if let Some(DiskNode::File(file_size, id)) =
                        self.insert_block.take().or_else(|| {
                            while let block @ Some(_) = self.blocks.pop_back() {
                                if let node @ Some(DiskNode::File(_, _)) = block {
                                    return node;
                                }
                            }
                            None
                        })
                    {
                        if file_size > 1 {
                            let _ = self.insert_block.insert(DiskNode::File(file_size - 1, id));
                        }
                        if *size > 1 {
                            let _ = self.current_block.insert(DiskNode::Free(size - 1));
                        }
                        return Some(id);
                    };
                    return None;
                }
            }
        } else if let Some(DiskNode::File(size, id)) = self.insert_block.take() {
            if size > 1 {
                let _ = self.insert_block.insert(DiskNode::File(size - 1, id));
            }
            return Some(id);
        }
        None
    }
}

aoc2024::main!("../../inputs/day_09.txt");

aoc2024::test!("2333133121414131402\n", 1928, 2858);

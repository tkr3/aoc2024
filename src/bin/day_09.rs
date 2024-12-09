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
    0
}

#[derive(Debug)]
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
        if let Some(node) = &self
            .current_block
            .take()
            .or_else(|| self.blocks.pop_front())
        {
            match node {
                DiskNode::File(size, id) => {
                    if *size > 1 {
                        let _ = self.current_block.insert(DiskNode::File(size - 1, *id));
                    }
                    return Some(*id);
                }
                DiskNode::Free(size) => {
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

aoc2024::test!(
    "\
2333133121414131402
",
    1928,
    0
);

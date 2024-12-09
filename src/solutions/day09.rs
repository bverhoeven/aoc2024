#[derive(Clone, Debug, PartialEq, Eq)]
enum Block {
    File(u64),
    Free,
}

fn parse_input(input: &Vec<String>) -> (Vec<Block>, u64) {
    let mut disk: Vec<Block> = vec![];
    let mut max_file_id = 0;

    input.first().unwrap().chars().enumerate().for_each(|(i, char)| {
        let length = char.to_digit(10).unwrap() as usize;
        let file_id = (i / 2) as u64;

        if i % 2 == 0 {
            disk.append(&mut vec![Block::File(file_id); length]);
        } else {
            disk.append(&mut vec![Block::Free; length]);
        }

        max_file_id = max_file_id.max(file_id);
    });

    (disk, max_file_id)
}

fn compress(disk: &Vec<Block>) -> Vec<Block> {
    let mut compressed = disk.clone();

    for (index, block) in disk.iter().enumerate().rev() {
        if let Block::File(file_id) = block {
            if let Some(free_index) = compressed.iter().position(|b| *b == Block::Free) {
                if free_index < index {
                    compressed[free_index] = Block::File(*file_id);
                    compressed[index] = Block::Free;
                }
            }
        }
    }

    compressed
}

fn find_free_space(blocks: &[Block], length: usize) -> Option<usize> {
    blocks
        .windows(length)
        .position(|window| window.iter().all(|block| *block == Block::Free))
}

fn compress_defragment(disk: &[Block], max_file_id: u64) -> Vec<Block> {
    let mut compressed = disk.to_vec();

    for file_id in (0..=max_file_id).rev() {
        if let Some(file_start) = compressed.iter().position(|block| *block == Block::File(file_id)) {
            let file_size = compressed
                .iter()
                .filter(|&block| *block == Block::File(file_id))
                .count();

            if let Some(free_start) = find_free_space(&compressed[..file_start], file_size) {
                for index in 0..file_size {
                    compressed[free_start + index] = Block::File(file_id);
                    compressed[file_start + index] = Block::Free;
                }
            }
        }
    }

    compressed
}

fn calculate_checksum(compressed: &[Block]) -> i64 {
    let mut checksum = 0;

    for (index, block) in compressed.iter().enumerate() {
        if let Block::File(file_id) = block {
            checksum += index as u64 * file_id;
        }
    }

    checksum as i64
}

pub fn part1(input: &Vec<String>) -> i64 {
    let (disk, _) = parse_input(input);
    calculate_checksum(&compress(&disk))
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (disk, max_file_id) = parse_input(input);
    calculate_checksum(&compress_defragment(&disk, max_file_id))
}

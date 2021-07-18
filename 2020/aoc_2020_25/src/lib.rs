#[cfg(test)]
mod tests;

const DIVIDEND: usize = 20201227;
const CARD_AND_DOOR_SUBJECT_NUMBER: usize = 7;

pub fn find_encryption_key(pub_key_1: usize, pub_key_2: usize) -> usize {
    let loop_size_1 = find_loop_size(pub_key_1);
    let encryption_key = transform_subject_number(pub_key_2, loop_size_1);
    encryption_key
}

fn find_loop_size(pub_key: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    while value != pub_key {
        value = iterate(value, CARD_AND_DOOR_SUBJECT_NUMBER);
        loop_size += 1;
    }
    loop_size
}

fn transform_subject_number(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = iterate(value, subject_number);
    }
    value
}

fn iterate(value: usize, subject_number: usize) -> usize {
    let value = value * subject_number;
    value % DIVIDEND
}

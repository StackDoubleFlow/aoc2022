use std::fs;
fn main() {
    let input = fs::read_to_string("./input2.txt").unwrap();
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let range = &chars[i..i + 14];
        let mut bitfield: u32 = 0;
        for &c in range {
            bitfield |= 1 << (c as usize - 'a' as usize);
        }
        if bitfield.count_ones() == 14 {
            let idx = i + 14;
            println!("Marker at {idx}");
            break;
        }
    }
}

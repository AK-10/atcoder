// test1
// a
// test2
// abcdfhijklmnopqrstuvwxyz
// test3
// qazplwsxokmedcijnrfvuhbgt

use std::io;

fn main() {
    let checked_list: &mut [bool; 26] = &mut [false; 26];
    let mut s: String = String::new();
    let _ = io::stdin().read_line(&mut s);
    // use slicing to remove last null char.
    s[0..s.len() - 1].chars().for_each(|c| {
        let index = (c as usize) - 97;
        checked_list[index] = true;
    });

    let non_exist = checked_list.iter().position(|i| !*i).unwrap() as u8;
    println!("{}", (non_exist + 97) as char)
}

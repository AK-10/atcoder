// test1
// 4
// ###.
// ..#.
// ..#.
// ..#.
// ...#
// ...#
// ###.
// ....

// test2
// 13
// .#..###..##..
// #.#.#..#.#.#.
// #.#.###..#...
// ###.#..#.#.#.
// #.#.###..##..
// .............
// ..#...#....#.
// .##..#.#..##.
// #.#..#.#.#.#.
// ####.#.#.####
// ..#..#.#...#.
// ..#...#....#.
// .............
// .............
// .#....#...#..
// .#...#.#..#..
// ####.#.#.####
// .#.#.###..#.#
// .##....#..##.
// .#....#...#..
// .............
// ..##..###.#.#
// .#.#.#..#.###
// .#.#..###.#.#
// .#.#.#..#.#.#
// ..##..###..#.

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        t: [String; n]
    }

    let mut s = s
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let t = t
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = usize::MAX;
    // rotate 3 times (if do 4 times, it is same as original)
    for k in 0..4 {
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                // swap marks
                if s[i][j] != t[i][j] {
                    cnt += 1;
                }
            }
        }

        // rotate
        s = (0..n)
            .map(|i| (0..n).map(|j| s[n - j - 1][i]).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        ans = ans.min(cnt + k);
    }

    println!("{}", ans);
}

// test 1
// 4 4
// 2 4
// 3 1
// 4 1
// 2 3
//
// Yes
// test2
//
// 4 6
// 1 2
// 1 3
// 1 4
// 2 3
// 2 4
// 3 4
//
// No

use ac_library::dsu::Dsu;
use proconio::input;
// this answer makes TLE.
// fn main() {
//     input! {
//         _: usize,
//         m: usize,
//         mut edges: [(usize, usize); m]
//     }
//
//     let first_item = edges.remove(0);
//     let start_vertex = first_item.0;
//     let mut next_vertex = first_item.1;
//
//     loop {
//         for (index, item) in edges.iter().enumerate() {
//             if item.0 == next_vertex {
//                 let next = edges.remove(index);
//                 next_vertex = next.1;
//                 break;
//             }
//             if item.1 == next_vertex {
//                 let next = edges.remove(index);
//                 next_vertex = next.0;
//                 break;
//             }
//         }
//         if start_vertex == next_vertex {
//             if edges.is_empty() {
//                 println!("Yes");
//                 break;
//             } else {
//                 println!("No");
//                 break;
//             }
//         }
//     }
// }

// degree: 2
// connected
fn main() {
    input! {
        n: usize,
        m: usize,
        mut edges: [(usize, usize); m]
    }
    // count degrees
    let mut degrees = vec![0usize; n];
    // use (Disjoint Set Union) union find
    let mut union_find = Dsu::new(n);
    edges.iter().for_each(|(a, b)| {
        degrees[a - 1] += 1;
        degrees[b - 1] += 1;
        union_find.merge(*a - 1, *b - 1);
    });
    if degrees.iter().all(|i| *i == 2) && (0..n).into_iter().all(|i| union_find.same(0, i)) {
        println!("Yes");
    } else {
        println!("No");
    }
}

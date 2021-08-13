#![cfg(not(miri))] // FIXME: takes too long

use hashbrown::HashSet;
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};

#[test]
fn test_hashset_insert_remove() {
    let mut m: HashSet<Vec<char>> = HashSet::new();
    //let num: u32 = 4096;
    //let tx: Vec<Vec<u8>> = (0..num).map(|i| (i..(16 + i)).collect()).collect();
    let seed: [u8; 16] = [
        130, 220, 246, 217, 111, 124, 221, 189, 190, 234, 121, 93, 67, 95, 100, 43,
    ];

    let rng = &mut SmallRng::from_seed(seed);
    let tx: Vec<Vec<char>> = (0..4096)
        .map(|_| (rng.sample_iter(&Alphanumeric).take(32).collect()))
        .collect();

    // more readable with explicit `true` / `false`
    #[allow(clippy::bool_assert_comparison)]
    for _ in 0..32 {
        for x in tx.iter() {
            assert_eq!(m.contains(x), false);
            assert_eq!(m.insert(x.clone()), true);
        }
        for (i, x) in tx.iter().enumerate() {
            println!("removing {} {:?}", i, x);
            assert_eq!(m.remove(x), true);
        }
    }
}

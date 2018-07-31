#![feature(nll)]
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut f = File::open("words.txt").unwrap();
    let mut words_string = String::new();
    f.read_to_string(&mut words_string).unwrap();
    
    let mut first_letter = HashMap::new();
    let mut letter_pairs = HashMap::new();
    let num_words = words_string.lines().count();
    for (wn, word) in words_string.lines().take(num_words).enumerate() {
        for i in 0.. {
            if let Some(lp) = word.get(i..i+2) {
                let lp_s = lp.chars().collect::<Vec<_>>();
                if lp_s[0] != lp_s[1] {
                    *letter_pairs.entry((lp_s[0], lp_s[1])).or_insert(0.) += 1.0 / (wn + 1) as f64;
                    // *letter_pairs.entry((lp_s[1], lp_s[0])).or_insert(0) += 1.0;
                }
            } else {
                break;
            }
        }

        let first_char = word.chars().next().unwrap();
        *first_letter.entry(first_char).or_insert(0.) += 1.0 / (wn + 1) as f64;
    }
    for c in 'a' as u8..='z' as u8 {
        for h in 'a' as u8..='z' as u8 {
            let (c, h) = (c as char, h as char);
            if c != h && !letter_pairs.get(&(c, h)).is_some() {
                letter_pairs.insert((c, h), 0.);
            }
        }
    }

    let mut lp_sorted_grouped = letter_pairs.iter().collect::<Vec<_>>();
    lp_sorted_grouped.sort_by_key(|lp| (&(lp.0).0, (lp.1 * 100000.) as u64));
    let mut lp_grouped = Vec::new();
    let mut last_char = ' ';
    for &(lp, count) in &lp_sorted_grouped {
        if lp.0 != last_char {
            lp_grouped.push(vec![(lp, count)]);
        } else {
            lp_grouped.last_mut().unwrap().push((lp, count));
        }
        last_char = lp.0;
    }
    let letter_ranks = "etaoinshrdlcumwfgypbvkjxqz";
    lp_grouped.sort_by_key(|lp_vec| 26 - letter_ranks.find((lp_vec[0].0).0).unwrap());

    let mut lp_sorted = letter_pairs.iter().collect::<Vec<_>>();
    lp_sorted.sort_by_key(|lp|(lp.1 * 100000.) as u64);
    for (lp, count) in &lp_sorted {
        println!("{}{} {:.4}", lp.0, lp.1, count);
    }

    // List view
    for v in &lp_grouped {
        for (lp, count) in v {
            println!("{}{} {:.4}", lp.0, lp.1, count);
        }
        println!();
    } // */

    let mut used_characters: HashSet<char> = HashSet::new();
    let mut unused_characters = HashSet::new();
    for c in ('a' as u8..='z' as u8).map(|c| c as char) {
        unused_characters.insert(c);
    }
    let mut wheels = vec![String::new(); 4];
    let wheel_max_len = 8;

    let mut whi = 0;
    while unused_characters.len() > 0 {
        let wheel = &mut wheels[whi];
        macro_rules! insert_char {
            ($c:expr) => {{
                let c = $c;
                wheel.push(c);
                used_characters.insert(c);
                unused_characters.remove(&c);
            }}
        }

        if wheel.len() == 0 {
            insert_char!(lp_sorted.iter().rev().find(|(lp, _)| !used_characters.contains(&lp.0)).map(|(lp, _)| lp.0).unwrap());
        } else {
            let mut char_ranks = HashMap::new();
            for c in &unused_characters {
                let mut rank = 0.0;
                for wc in wheel.chars() {
                    rank += letter_pairs.get(&(*c, wc)).unwrap();
                    rank += letter_pairs.get(&(wc, *c)).unwrap();
                }
                char_ranks.insert(*c, rank);
            }
            {
                print!("{}\n  " , wheel);
                let mut cr_sorted = char_ranks.iter().collect::<Vec<_>>();
                cr_sorted.sort_by_key(|&(_, r)| (r * 10000.) as u64);
                for (c, _) in cr_sorted {
                    print!("{}", c);
                }
                println!();
            }

            let mut lowest_char = ' ';
            let mut lc_rank = 1.0/0.0;
            for (c, rank) in char_ranks {
                if rank < lc_rank {
                    lowest_char = c;
                    lc_rank = rank;
                }
            }
            assert_ne!(lowest_char, ' ');
            insert_char!(lowest_char);
        }
        whi += 1;
        whi %= wheels.len();
        if whi == 0 {
            println!();
        }
    }

    /* 
    for v in lp_grouped.into_iter().rev() {
        let lead_char = (v[0].0).0;
        macro_rules! active_wheel {
            () => {{
                let mut changed = false;
                if wheels.last_mut().unwrap().len() == wheel_max_len {
                    wheels.push(String::new());
                    changed = true;
                }
                (wheels.last_mut().unwrap(), changed)
            }}
        };
        if !used_characters.contains(&lead_char) {
            active_wheel!().0.push(lead_char);
            used_characters.insert(lead_char);
        }

        for c in v.iter().map(|g| (g.0).1) {
            let (active_wheel, changed) = active_wheel!();
            if changed {
                break;
            }
            if !used_characters.contains(&c) {
                active_wheel.push(c);
                used_characters.insert(c);
            }
        }
    } // */

    let mut fl_sorted = first_letter.into_iter().collect::<Vec<_>>();
    fl_sorted.sort_by_key(|f| (-f.1 * 10000.) as i64);
    for (l, _) in fl_sorted {
        print!("{}", l);
    }
    println!("\n");

    for w in wheels {
        println!("{}", w);
    }
    // */
}

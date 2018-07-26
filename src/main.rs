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
    let mut lp_sorted = letter_pairs.iter().collect::<Vec<_>>();
    lp_sorted.sort_by_key(|lp| (&(lp.0).0, (lp.1 * 100000.) as u64));
    let mut lp_grouped = Vec::new();
    let mut last_char = ' ';
    for &(lp, count) in &lp_sorted {
        if lp.0 != last_char {
            lp_grouped.push(vec![(lp, count)]);
        } else {
            lp_grouped.last_mut().unwrap().push((lp, count));
        }
        last_char = lp.0;
    }
    let letter_ranks = "etaoinshrdlcumwfgypbvkjxqz";
    lp_grouped.sort_by_key(|lp_vec| 26 - letter_ranks.find((lp_vec[0].0).0).unwrap());

    // List view
    for v in &lp_grouped {
        for (lp, count) in v {
            println!("{}{} {:.4}", lp.0, lp.1, count);
        }
        println!();
    } // */

    let mut used_characters: HashSet<char> = HashSet::new();
    let mut wheels = vec![String::new()];
    let wheel_max_len = 8;

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
    }

    let mut fl_sorted = first_letter.into_iter().collect::<Vec<_>>();
    fl_sorted.sort_by_key(|f| (-f.1 * 10000.) as i64);
    for (l, _) in fl_sorted {
        print!("{}", l);
    }
    println!("\n");

    for w in wheels {
        println!("{}", w);
    }
}

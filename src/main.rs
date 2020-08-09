mod freq;

use anyhow::Result;
use async_std::fs::File;
use async_std::prelude::*;
use async_std::task;
use freq::{merge_freq_maps, FreqGetter, FreqMap};
use glob::glob;
use std::collections::HashSet;
use std::path::PathBuf;
use std::env;

async fn get_file_freq(path: PathBuf, getter: FreqGetter) -> Result<FreqMap> {
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(getter.get(&content))
}

fn get_filter_set() -> HashSet<char> {
    // vec![
    //     '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
    //     '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+',
    // ]
        vec![
            '\'', '"'
        ]
        .into_iter()
        .collect()
}

#[async_std::main]
async fn main() -> Result<()> {
    let sym_or_num_set = get_filter_set();
    let glob_str = env::args().nth(1).expect("Missing glob pattern");
    let glb = glob(&glob_str).expect("Bad glob");
    let mut children = vec![];

    for entry in glb {
        match entry {
            Ok(path) => {
                let getter = FreqGetter::new().filter_set(sym_or_num_set.clone());
                let child = task::spawn(async move {
                    get_file_freq(path.clone(), getter)
                        .await
                        .map(|freq| (path, freq))
                });
                children.push(child);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }

    let mut merged = FreqMap::new();

    for child in children {
        let (path, freq) = child.await?;
        println!("{}", path.display());
        merge_freq_maps(&mut merged, &freq);
    }

    println!("\n-------------- RESULT --------------\n");

    let mut sorted: Vec<(char, u32)> = merged.into_iter().collect();
    sorted.sort_by(|(_, v), (_, v2)| v2.cmp(v));

    for (key, value) in sorted.iter() {
        println!("{}: {}", key, value);
    }

    Ok(())
}

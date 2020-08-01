mod freq;

use anyhow::Result;
use async_std::fs::File;
use async_std::prelude::*;
use async_std::task;
use async_std::sync::{Arc, Mutex};
use freq::{FreqGetter, FreqMap};
use regex::Regex;
use std::path::PathBuf;
use glob::glob;

async fn get_file_freq(path: PathBuf, getter: FreqGetter) -> Result<FreqMap> {
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(getter.get(&content))
}

#[async_std::main]
async fn main() -> Result<()> {
    let sym_or_num_re = Regex::new(r"[0-9!@#$%^&*()_+-=]").expect("Bad regex");
    let glb = glob("/home/igarin/workspace/**/*.rs").expect("Bad glob");
    let mut children = vec![];
    let mut maps = Arc::new(Mutex::new(Vec::<(PathBuf, FreqMap)>::new()));

    for entry in glb {
        match entry {
            Ok(path) => {
                let getter = FreqGetter::new().filter_regex(sym_or_num_re.clone());
                let priv_maps = maps.clone();
                let child = task::spawn(async move {
                    let freq = get_file_freq(path.clone(), getter).await.unwrap();
                    let mut m = priv_maps.lock().await;
                    m.push((path, freq));
                });
                children.push(child);
            }
            Err(e) => { dbg!(e); }
        }
    }

    for child in children {
        child.await;
    }

    for map in maps.lock().await.iter() {
        println!("{}: (: {}",
                 map.0.display(),
                 map.1.get(&'(').unwrap_or(&0));
    }

    Ok(())
}

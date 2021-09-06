use mapreduce_rs::{Maper, Reducer};
use std::{
    collections::HashMap,
    env::current_dir,
    error::Error,
    fs::{self, remove_file, File, OpenOptions},
    io::{BufReader, Write},
    path::PathBuf,
};
struct Mapers {
    id: u32,
    file: PathBuf,
}
impl Maper for Mapers {
    fn map(&self) -> Result<(), Box<dyn Error>> {
        let path = current_dir()?.join(&self.file);
        if !path.exists() {
            println!("file don't exist!");
            return Ok(());
        }

        let contents = fs::read_to_string(&path).unwrap().to_ascii_lowercase();
        let contents: String = contents
            .matches(|c: char| c.is_ascii_alphabetic() || c.is_whitespace())
            .collect();
        let words: Vec<&str> = contents.split_ascii_whitespace().collect();
        let mut wc = HashMap::new();

        for i in words {
            if wc.contains_key(i) {
                let t = wc.get_mut(i).unwrap();
                *t += 1;
            } else {
                wc.insert(i, 1);
            }
        }
        let mut list0 = Vec::new();
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        for i in wc {
            let t = i.0.as_bytes();
            let fi_num = (t[0] - 97) % 26;
            if fi_num <= 9 {
                list0.push(i);
            } else if fi_num <= 18 {
                list1.push(i);
            } else {
                list2.push(i);
            }
        }
        self.send(0, list0)?;
        self.send(1, list1)?;
        self.send(2, list2)?;
        Ok(())
    }
    fn send(&self, id: u32, list: Vec<(&str, i32)>) -> Result<(), Box<dyn Error>> {
        let file = current_dir()?.join(format!("{}-{}", id, self.id));
        if !file.exists() {
            File::create(&file)?;
        }
        let mut buffer = OpenOptions::new().append(true).open(&file)?;
        let entry = serde_json::to_string(&list)?;
        buffer.write_all(entry.as_bytes())?;
        buffer.flush()?;

        Ok(())
    }
    fn recv(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct Reducers {
    id: u32,
}

impl Reducer for Reducers {
    fn reduce(&self) -> Result<(), Box<dyn Error>> {
        let path = current_dir()?;
        let mut store = HashMap::new();
        let mut list = Vec::new();
        for entry in fs::read_dir(&path)? {
            let entry = entry?.path();
            let file = match entry.file_name() {
                Some(x) => x,
                _ => continue,
            };
            let file = file.to_str().unwrap();
            if file.starts_with(format!("{}", self.id).as_str()) {
                list.push(entry);
            }
        }

        for i in list {
            let file = i.clone();
            let reader = BufReader::new(File::open(i)?);
            let words: Vec<(String, i32)> = serde_json::from_reader(reader)?;
            remove_file(file)?;
            for i in words {
                if store.contains_key(&i.0) {
                    let value = store.get_mut(&i.0).unwrap();
                    *value += 1;
                } else {
                    store.insert(i.0, i.1);
                }
            }
        }

        let path = path.join(format!("{}.txt", self.id));
        let mut file = OpenOptions::new().append(true).create(true).open(&path)?;
        for i in store {
            let entry = serde_json::to_string(&i)?;
            file.write_all(entry.as_bytes())?
        }
        file.flush()?;

        Ok(())
    }
    fn recv(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let file = current_dir()?.join("many_books.txt");

    let maper = Mapers { id: 1, file: file };
    maper.map()?;
    let reduce = Reducers { id: 1 };
    reduce.reduce()?;

    Ok(())
}

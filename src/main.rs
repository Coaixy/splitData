use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

struct splitData {
    data: Vec<u8>,
    path: String,
}

impl splitData {
    pub fn new(path: &str) -> Self {
        splitData {
            data: Vec::new(),
            path: path.to_string(),
        }
    }

    fn read_data(&mut self) {
        let mut file = File::open(&self.path).expect("Failed to open file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .expect("Failed to read file");
        let dir_path = "data_".to_owned() + &self.path.replace(".", "-");
        if fs::metadata(&dir_path).is_ok() {
            fs::remove_dir_all(&dir_path).expect("Failed to remove all files");
            fs::create_dir(&dir_path).expect("Failed to create dir");
        } else {
            fs::create_dir(&dir_path).expect("Failed to create dir");
        }
        self.data = contents;
        self.path = self.path.to_string();
    }

    pub fn split_data(&mut self, count: usize) {
        self.read_data();
        let length = self.data.len();
        let every_length = length / count;
        for i in 1..(count) {
            let mut data: Vec<u8> = Vec::new();
            for j in (i - 1) * every_length..i * every_length {
                data.push(*self.data.get(j).unwrap());
            }
            let path =
                "data_".to_owned() + &self.path.replace(".", "-") + "\\" + &i.to_string() + ".dat";
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&path)
                .unwrap();
            file.write_all(&data).expect("Failed to write");
        }
        let mut data: Vec<u8> = Vec::new();
        let path =
            "data_".to_owned() + &self.path.replace(".", "-") + "\\" + &count.to_string() + ".dat";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();
        for i in (count - 1) * every_length..length {
            data.push(*self.data.get(i).unwrap());
        }
        file.write_all(&data).unwrap();
    }
    pub fn merge_data(&mut self,ext:&str) {
        let dir_path = "data_".to_owned() + &self.path.replace(".", "-");
        let mut file_names: Vec<_> = fs::read_dir(&dir_path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        file_names.sort();
        let mut all_data:Vec<u8> = Vec::new();
        for i in file_names{
            let path = dir_path.to_string() +"\\"+ &i.to_string_lossy().into_owned();
            let data = fs::read(path).unwrap();
            all_data.extend(data);
        }
        let file_path = self.path.to_string() + &ext.to_string();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        file.write_all(&all_data).unwrap();
    }
}
fn main() {
    let mut sd = splitData::new("1.jpg");
    sd.split_data(4);
    sd.merge_data(".sd");
}

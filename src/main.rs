use std::{
    env,
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
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
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
    pub fn merge_data(&mut self, dir_path: &str, ext: &str) {
        let dir_path = dir_path.to_string();
        let mut file_names: Vec<_> = fs::read_dir(&dir_path)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        file_names.sort_by(|a, b| {
            let a_num: i32 = a
                .to_string_lossy()
                .to_string()
                .split('.')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let b_num: i32 = b
                .to_string_lossy()
                .to_string()
                .split('.')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            a_num.cmp(&b_num)
        });
        let mut all_data: Vec<u8> = Vec::new();
        for i in file_names {
            let path = dir_path.to_string() + "\\" + &i.to_string_lossy().into_owned();
            let data = fs::read(path).unwrap();
            all_data.extend(data);
        }
        let file_path = dir_path.replace("-", ".").replace("data_", "") + &ext.to_string();
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
    let args: Vec<String> = env::args().collect();
    let mut sd = splitData::new("");
    let this_dir = env::current_dir().unwrap();
    //Test
    if args.len() == 1 {
        sd.set_path("1.jpg");
        let dir_path = this_dir.to_string_lossy().to_string()
            + "\\"
            + &"data_".to_string()
            + &"1.jpg".to_string().replace(".", "-");
        sd.split_data(10);
        sd.merge_data(&dir_path, "");
    //Test
    } else if args.len() <= 2 {
        panic!("not match args");
    } else {
        match args.get(1).unwrap().as_str() {
            "-s" => {
                sd.set_path(args.get(2).unwrap());
                let count = args.get(3).unwrap().parse::<usize>().unwrap();
                sd.split_data(count);
            }
            "-m" => {
                let file_name = args.get(2).unwrap();
                sd.set_path(file_name);

                let dir_path = args.get(2).unwrap();
                let path = Path::new(dir_path);
                let path = if path.is_absolute() {
                    dir_path.replace(".", "-").to_string()
                } else {
                    this_dir.to_str().unwrap().to_owned() + "\\" + dir_path
                };
                let ext = if args.len() == 4 {
                    args.get(3).unwrap()
                } else {
                    ""
                };
                sd.merge_data(&dir_path, ext);
            }
            _ => {}
        }
    }
}

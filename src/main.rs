use std::{fs::{File, self}, io::{Read, Write}};

struct splitData{
    data:Vec<u8>,
    path:String
}

impl splitData {
    pub fn new() -> Self{
        splitData { data: Vec::new(), path: "".to_string()}
    }
    
    pub fn read_data(mut self,path:&str){
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read file");
    
        //如果文件夹存在就删除全部内容，不存在则创建
        let dir_path = "data_".to_owned()+path;
        if fs::metadata(&dir_path).is_ok() {
            fs::remove_dir_all(&dir_path);
        }else{
            fs::create_dir(&dir_path);
        }
        self.data = contents;
        self.path = path.to_string();
    }

    pub fn split_data(self,data:Vec<u8>){
        
    }
}
fn main() {
    let mut sd = splitData::new();
}


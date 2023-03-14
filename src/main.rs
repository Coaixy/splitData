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
            fs::remove_dir_all(&dir_path).expect("Failed to remove all files");
        }else{
            fs::create_dir(&dir_path).expect("Failed to create dir");
        }
        self.data = contents;
        self.path = path.to_string();
    }

    pub fn split_data(mut self,count:usize){
        let length = self.data.len();
        let every_length = length/count;
        //平均分配数据
        for i in 1..count-1{
            for j in (count-1)*every_length..count*every_length-1{

            }
        }
        //剩余的数据
        for i in (count-1)*every_length..length{

        }
    }
    pub fn merge_data(mut self){
        
    }
}
fn main() {
    let mut sd = splitData::new();
}


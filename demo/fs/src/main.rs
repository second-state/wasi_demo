use std::fs;

fn main() {
    fs::create_dir("./tmp1").unwrap();
    fs::create_dir("./tmp2").unwrap();
    let r = fs::read_dir(".").unwrap();
    for e in r {
        println!("{e:?}");
    }

    fs::write("./tmp1/data1", "data").unwrap();
    fs::rename("./tmp1/data1", "./tmp2/data2").unwrap();

    fs::write("./tmp1/data1", "data").unwrap();
    fs::write("./tmp1/data2", "data").unwrap();
    fs::remove_dir_all("./tmp1").unwrap();
}

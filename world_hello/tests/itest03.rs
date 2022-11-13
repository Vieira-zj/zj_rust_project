//
// Vector
//
// TODO:

//
// 错误处理
//

#[test]
fn it_error_handle() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_from_file() -> Result<String, io::Error> {
        let mut f = File::open("/tmp/test/log.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    match read_from_file() {
        Ok(s) => println!("read content:\n{}", s),
        Err(e) => println!("read error: {}", e),
    };
}

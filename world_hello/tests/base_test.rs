use std::io::{BufRead, Read};

// iterator

#[test]
fn it_handle_err_from_collect() {
    use std::convert::TryFrom;

    let input: Vec<i64> = vec![0, 1, 2, 3, 4, 512];
    let result = input
        .into_iter()
        .map(|v| <u8>::try_from(v))
        .collect::<Result<Vec<_>, _>>();
    match result {
        Ok(v) => println!("vec<u8>: {:?}", v),
        Err(err) => println!("get error: {:?}", err),
    }
}

// trait

#[test]
fn it_copy_trait() {
    #[derive(Debug, Clone, Copy)]
    struct KeyId(u32);
    let k = KeyId(42);
    let copied = k; // value bitwise copied from k to copied
    println!("src key: {:?}", k);
    println!("copied key: {:?}", copied);
}

// ref

#[test]
fn it_box_deref() {
    struct Point {
        x: i32,
        y: i32,
    }

    fn show(pt: &Point) {
        println!("x={}, y={}", pt.x, pt.y);
    }

    let pt = Point { x: 1, y: 3 };
    let ref_pt = &pt;
    show(ref_pt);

    // deref creates a reference to the Target type.
    let box_pt = Box::new(pt);
    show(&box_pt);
}

// custom err for error handle

#[test]
fn it_custom_string_error() {
    #[derive(Debug)]
    struct MyError(String);

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for MyError {}

    impl std::convert::From<String> for MyError {
        fn from(content: String) -> Self {
            Self(content)
        }
    }

    fn read_file(fpath: &str) -> Result<String, MyError> {
        let mut s = String::new();
        let mut f =
            std::fs::File::open(fpath).map_err(|err| format!("fail open file: {:?}", err))?;
        // 需要引入 std::io::Read 特征
        f.read_to_string(&mut s)
            .map_err(|err| format!("fail read: {:?}", err))?;
        Ok(s)
    }

    match read_file("/tmp/test/nonexist.txt") {
        Ok(content) => {
            println!("read file:\n{}", content);
        }
        Err(err) => {
            println!("get error: {}", err);
        }
    }
}

#[test]
fn it_custom_nested_error() {
    #[derive(Debug)]
    enum MyError {
        Io(std::io::Error),
        Utf8(std::string::FromUtf8Error),
        General(String),
    }

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyError::Io(e) => write!(f, "IO error: {}", e),
                MyError::Utf8(e) => write!(f, "utf-8 error: {}", e),
                MyError::General(s) => write!(f, "general error: {}", s),
            }
        }
    }

    impl std::error::Error for MyError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                MyError::Io(e) => Some(e),
                MyError::Utf8(e) => Some(e),
                MyError::General(_) => None,
            }
        }
    }

    impl From<std::io::Error> for MyError {
        fn from(e: std::io::Error) -> Self {
            Self::Io(e)
        }
    }
    impl From<std::string::FromUtf8Error> for MyError {
        fn from(e: std::string::FromUtf8Error) -> Self {
            Self::Utf8(e)
        }
    }

    fn read_first_line(fpath: &str) -> Result<String, MyError> {
        let f = std::fs::File::open(fpath)?;
        let mut reader = std::io::BufReader::new(f);
        let mut buf = vec![];
        let len = reader.read_until(b'\n', &mut buf)?;
        if len > 1024 {
            return Err(MyError::General(format!("line too long: {}", len)));
        }
        let result = String::from_utf8(buf)?;
        Ok(result)
    }

    let result = read_first_line("/tmp/test/nonexist.txt");
    match result {
        Ok(line) => println!("read first line:\n{}", line),
        Err(err) => println!("fail read:\n{}", err),
    }
}

#[test]
fn it_custom_wrapped_error() {
    /// custom wrapped error by trait object.
    #[derive(Debug)]
    enum MyWrappedError {
        Wrapped(Box<dyn std::error::Error>),
        General(String),
    }

    impl std::fmt::Display for MyWrappedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyWrappedError::Wrapped(err) => write!(f, "inner error: {}", err),
                MyWrappedError::General(s) => write!(f, "general error: {}", s),
            }
        }
    }

    fn read_first_line(fpath: &str) -> Result<String, MyWrappedError> {
        let f = std::fs::File::open(fpath).map_err(|err| MyWrappedError::Wrapped(Box::new(err)))?;
        let mut reader = std::io::BufReader::new(f);
        let mut buf = vec![];
        let len = reader
            .read_until(b'\n', &mut buf)
            .map_err(|err| MyWrappedError::Wrapped(Box::new(err)))?;
        if len > 1024 {
            return Err(MyWrappedError::General(format!("line to long: {}", len)));
        }

        let result =
            String::from_utf8(buf).map_err(|err| MyWrappedError::Wrapped(Box::new(err)))?;
        Ok(result)
    }

    let result = read_first_line("/tmp/test/rust.txt");
    match result {
        Ok(line) => println!("read first line:\n{}", line),
        Err(err) => println!("fail read:\n{}", err),
    }
}

// chain build

#[test]
#[allow(dead_code, deprecated)]
fn it_chain_build_of_struct() {
    #[derive(Debug)]
    struct Details {
        given_name: String,
        family_name: String,
        preferred_name: Option<String>,
        mobile_phone: Option<String>,
        dob: chrono::Date<chrono::Utc>,
        last_seen: Option<chrono::DateTime<chrono::Utc>>,
    }

    struct DetailsBuilder(Details);

    impl DetailsBuilder {
        fn new(given_name: &str, family_name: &str, dob: chrono::Date<chrono::Utc>) -> Self {
            DetailsBuilder(Details {
                given_name: given_name.to_owned(),
                family_name: family_name.to_owned(),
                preferred_name: None,
                mobile_phone: None,
                dob: dob,
                last_seen: None,
            })
        }

        fn preferred_name(&mut self, preferred_name: &str) -> &mut Self {
            self.0.preferred_name = Some(preferred_name.to_owned());
            self
        }

        fn mobile_phone(&mut self, mobile_phone: &str) -> &mut Self {
            self.0.mobile_phone = Some(mobile_phone.to_owned());
            self
        }

        fn just_seen(&mut self) -> &mut Self {
            self.0.last_seen = Some(chrono::Utc::now());
            self
        }

        fn build(self) -> Details {
            self.0
        }
    }

    let mut builder = DetailsBuilder::new("Bar", "Builder", chrono::Utc::today());
    builder.preferred_name("Foo").just_seen();
    let details = builder.build();
    println!("details: {:?}", details);
}

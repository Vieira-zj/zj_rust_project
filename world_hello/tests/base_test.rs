use std::io::{BufRead, Read};

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

// Iterator, IntoIterator

#[derive(Debug)]
struct Todo {
    message: String,
    done: bool,
}

struct Todos {
    list: Vec<Todo>,
}

// Iterator 迭代器

struct TodosIterator<'a> {
    todos: &'a Todos,
    index: usize,
}

impl<'a> Iterator for TodosIterator<'a> {
    type Item = &'a Todo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.todos.list.len() {
            let result = Some(&(self.todos.list[self.index]));
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl Todos {
    fn iter(&self) -> TodosIterator {
        TodosIterator {
            todos: self,
            index: 0,
        }
    }
}

#[test]
fn it_iterator_iter() {
    let list = vec![
        Todo {
            message: String::from("java"),
            done: true,
        },
        Todo {
            message: String::from("rust"),
            done: false,
        },
    ];
    let todos = Todos { list: list };

    // 引用的方式
    println!("todos:");
    for todo in todos.iter() {
        println!("{}: {}", todo.message, todo.done);
    }
}

// IntoIterator 可迭代对象

struct TodosIntoIterator {
    todos: Todos,
}

impl Iterator for TodosIntoIterator {
    type Item = Todo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.todos.list.len() == 0 {
            return None;
        }
        let result = self.todos.list.remove(0);
        Some(result)
    }
}

impl IntoIterator for Todos {
    type Item = Todo;
    type IntoIter = TodosIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TodosIntoIterator { todos: self }
    }
}

#[test]
fn it_iterator_into_iter() {
    let list = vec![
        Todo {
            message: String::from("python"),
            done: true,
        },
        Todo {
            message: String::from("rust"),
            done: false,
        },
    ];
    let todos = Todos { list: list };

    // 获取所有权的方式
    println!("todos:");
    for todo in todos {
        println!("{}: {}", todo.message, todo.done);
    }
}

// sort collection

#[test]
fn it_sort_vector() {
    // int
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // float
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

#[test]
fn it_sort_for_custom_struct() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Self {
            Person {
                name: name,
                age: age,
            }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort_by(|a, b| b.age.cmp(&a.age));
    for p in people.into_iter() {
        println!("{:?}", p);
    }
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

// fromstr

#[test]
fn it_new_struct_fromstr() {
    use std::str::FromStr;

    #[derive(Debug)]
    struct RGB {
        r: u8,
        g: u8,
        b: u8,
    }

    impl FromStr for RGB {
        type Err = std::num::ParseIntError;

        fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
            let r = u8::from_str_radix(&hex_code[1..3], 16)?;
            let g = u8::from_str_radix(&hex_code[3..5], 16)?;
            let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;
            Ok(RGB { r: r, g: g, b: b })
        }
    }

    let code = "#fa7268";
    match RGB::from_str(code) {
        Ok(rgb) => println!(
            r"The RGB color code is: R: {} G: {} B: {}",
            rgb.r, rgb.g, rgb.b
        ),
        Err(_) => println!("{} is not a valid color hex code", code),
    }
}

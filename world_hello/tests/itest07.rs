//
// Exp
//

#[test]
fn it_slice_and_vec() {
    // 读取使用 &[T] 而不是 &Vec<T>
    fn find_number(nums: &[i32], dst: i32) -> Option<&i32> {
        println!("numbers: {:?}", nums);
        nums.iter().find(|&&x| x == dst)
    }

    let mut nums = vec![1, 2, 3];
    nums.push(10);

    match find_number(&nums, 11) {
        Some(value) => println!("find item: {}", value),
        None => println!("item not found"),
    }
}

#[test]
fn it_iterator_slice() {
    fn largest_by_ref(values: &[i32]) -> &i32 {
        let mut largest = &values[0];
        for val in values {
            if val > largest {
                largest = val;
            }
        }
        return largest;
    }

    fn largest_by_copy(values: &[i32]) -> i32 {
        let mut largest = values[0];
        for &val in values {
            if val > largest {
                largest = val;
            }
        }
        largest
    }

    let v = [1, 4, 5, 2, 3];
    let result = largest_by_ref(&v);
    println!("largest: {}", result);

    let result = largest_by_copy(&v);
    println!("largest: {}", result);
}

#[test]
fn it_mut_borrow_01() {
    let mut x = 1;
    let y = &x;
    println!("{}, {}", x, y);

    let z = &mut x;
    // error: cannot assign to "x" because it is borrowed
    // x = 2;
    // println!("{}", x);
    *z = 3;
    println!("{}", z);

    let s = "hello";
    let sub = &s[..3];
    println!("{}, {}", s, sub);
}

#[test]
fn it_mut_borrow_02() {
    fn add_item(data: &mut Vec<i32>) {
        data.push(6);
    }

    let mut data = vec![1, 2, 3];
    data.push(4);
    {
        let bow = &mut data;
        bow.push(5);
    }
    add_item(&mut data);

    println!("{:?}", data);
}

#[test]
fn it_return_fn_local_str() {
    fn get_str<'a>() -> &'a str {
        // s 的作用域为 get_str 函数，而字符串字面量 "hello" 的生命周期是 'static
        // error
        // let s = String::from("hello");
        // return s.as_str();

        // ok
        let s = "hello";
        return s;
    }

    println!("{}", get_str());
}

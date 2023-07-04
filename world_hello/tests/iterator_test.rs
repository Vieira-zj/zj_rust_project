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

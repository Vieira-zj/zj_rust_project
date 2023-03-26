// global config

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    a: String,
    b: String,
}

pub static mut CONFIG: Option<&mut Config> = None;

pub fn init_config() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    // 将 c 从内存中泄漏，变成 'static 生命周期
    Some(Box::leak(c))
}

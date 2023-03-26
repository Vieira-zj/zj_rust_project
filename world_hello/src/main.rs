use world_hello::config;
use world_hello::tutorial::{fs_demo, macro_demo, rs_demo};

fn main() {
    unsafe {
        config::CONFIG = config::init_config();
        println!("{:?}", config::CONFIG);
    }

    run_demo(false);
    run_custom_macro_sample(false);

    // test:
    // cargo run -- body /tmp/test/poem.txt
    // IGNORE_CASE=1 cargo run -- to /tmp/test/poem.txt
    run_app_minigrep(true);

    println!("done");
}

fn run_demo(is_run: bool) {
    if !is_run {
        return;
    }

    rs_demo::greet_world();
    rs_demo::word_count();

    let word = rs_demo::first_word("hello world");
    println!("first word: {}", word);

    rs_demo::string_parse();
    rs_demo::get_value_by_input_index();

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    rs_demo::retain_even(&mut v);
    println!("even num: {:?}", v);

    fs_demo::read_file_sample_01();
    fs_demo::read_file_sample_02();

    rs_demo::display_trait_sample();
}

fn run_custom_macro_sample(is_run: bool) {
    if is_run {
        macro_demo::custom_macro_sample();
    }
}

fn run_app_minigrep(is_run: bool) {
    if is_run {
        use world_hello::apps::minigrep::app;
        app::run();
    }
}

use world_hello::config;
use world_hello::tutorial::{fs_demo, macro_demo, rs_demo};

fn main() {
    unsafe {
        config::init();
        println!("{:?}", config::CONFIG);
    }

    run_demo();
    run_apps();

    println!("done");
}

fn run_demo() {
    if true {
        rs_demo::greet_world();
        return;
    }

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

    macro_demo::custom_macro_sample();
}

fn run_apps() {
    // test:
    // cargo run -- body /tmp/test/poem.txt
    // IGNORE_CASE=1 cargo run -- to /tmp/test/poem.txt
    run_app_minigrep(false);

    // test:
    // http://127.0.0.1:7878
    run_app_webserver(false);
}

fn run_app_minigrep(is_run: bool) {
    if is_run {
        use world_hello::apps::minigrep::app;
        app::run();
    }
}

fn run_app_webserver(is_run: bool) {
    if is_run {
        use world_hello::apps::webserver::app_v1 as app;
        app::tcp_srv();
    }
}

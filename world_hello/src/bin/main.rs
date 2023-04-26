use world_hello::config;
use world_hello::tutorial::{fs_demo, macro_demo, rs_demo};

fn main() {
    config::init();
    update_config(false);

    run_demo();
    run_apps();
    println!("main done");
}

fn update_config(is_update: bool) {
    if is_update {
        unsafe {
            if let Some(cfg) = &mut config::CONFIG {
                // let cfg: &mut config::Config = &mut **cfg;
                println!("old: {:?}", cfg);
                cfg.b = "newB".to_string();
                println!("new config: a={},b={}", cfg.a, cfg.b);
            }
        }
    }
}

fn run_demo() {
    rs_demo::greet_world(true);
    rs_demo::get_value_by_input_index(false);

    rs_demo::word_count(false);
    rs_demo::first_word("hello world", false);
    rs_demo::text_parse(false);

    if false {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        rs_demo::retain_even_numbers(&mut v);
        println!("even num: {:?}", v);
    }

    rs_demo::display_trait(false);

    fs_demo::read_file_v1(false);
    fs_demo::read_file_v2(false);

    macro_demo::custom_macro(false);
}

fn run_apps() {
    // test:
    // cargo run -- body /tmp/test/poem.txt
    // IGNORE_CASE=1 cargo run -- to /tmp/test/poem.txt
    run_app_minigrep(false);

    // test:
    // curl http://127.0.0.1:7878/
    run_app_websrv(false);

    // test:
    // curl http://127.0.0.1:7878/
    // curl http://127.0.0.1:7878/sleep
    run_app_websrv_parallel(false);

    run_app_aysnc(false);
}

fn run_app_minigrep(is_run: bool) {
    if is_run {
        use world_hello::apps::minigrep::app;
        app::run();
    }
}

fn run_app_websrv(is_run: bool) {
    if is_run {
        use world_hello::webserver::app_v1 as app;
        app::tcp_srv();
    }
}

fn run_app_websrv_parallel(is_run: bool) {
    if is_run {
        use world_hello::webserver::app_v2 as app;
        app::tcp_srv();
    }
}

fn run_app_aysnc(is_run: bool) {
    if is_run {
        use world_hello::runasync::app;
        app::start();
    }
}

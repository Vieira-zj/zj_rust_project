use world_hello::config;
use world_hello::tutorial::{calculator, download, fsdemo, macrodemo, rsdemo};

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
    rsdemo::greet_world(true);
    rsdemo::get_value_by_input_index(false);

    rsdemo::word_count(false);
    rsdemo::first_word("hello world", false);
    rsdemo::text_parse(false);

    if false {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        rsdemo::retain_even_numbers(&mut v);
        println!("even num: {:?}", v);
    }

    rsdemo::display_trait(false);

    fsdemo::read_file_v1(false);
    fsdemo::read_file_v2(false);

    macrodemo::custom_macro(false);

    if false {
        calculator::run();
    }
    if false {
        download::run();
    }
}

fn run_apps() {
    run_app_aysnc_executor(false);

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
    run_app_parallel_websrv(false);
}

fn run_app_minigrep(is_run: bool) {
    if is_run {
        use world_hello::apps::minigrep::app;
        app::run();
    }
}

fn run_app_websrv(is_run: bool) {
    if is_run {
        use world_hello::webserver::appv1 as app;
        app::tcp_srv();
    }
}

fn run_app_parallel_websrv(is_run: bool) {
    if is_run {
        use world_hello::webserver::appv2 as app;
        app::tcp_srv();
    }
}

fn run_app_aysnc_executor(is_run: bool) {
    if is_run {
        use world_hello::executor::app;
        app::start();
    }
}

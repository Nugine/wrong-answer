use wa_server::config::GLOBAL_CONFIG;
use wa_server::hello;

fn main() {
    println!("{}", GLOBAL_CONFIG.redis_url);
    hello();
}

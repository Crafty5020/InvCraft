

fn main() {
    println!("Hello, MINECRAFTJHJC GHJGFJGJFHEJHFJHEJKNHKFBEJBFJBUKJFBUHEJFBJB CAINE WAS HERE! HHFIUHEIUHFIUHISHFUI!");
    let cur_engine: Result<engine::Engine, common::ERROR> = engine::Engine::init("Inventory Craft", 1280, 720);

    if cur_engine.is_err() {
        println!("Failed to initialize engine: {:?}", cur_engine.err());
        return;
    }

    let cur_engine: engine::Engine = cur_engine.unwrap();
    while cur_engine.is_running() {
        
    }

    cur_engine.stop();
}

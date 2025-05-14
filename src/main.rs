fn greet_world() {
    let chinese = "你好世界";
    let southern_germany = "Grüß Gott";
    let english = "Hello, world!";
    let french = "Bonjour le monde";        
    let spanish = "Hola, mundo";
    let italian = "Ciao, mondo";
    let regions = [southern_germany, chinese, english,french, spanish, italian];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
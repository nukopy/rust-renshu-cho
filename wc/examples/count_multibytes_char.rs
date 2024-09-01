fn main() {
    let s = "y̆";
    let char_count = s.chars().count();
    println!("Character count: {}", char_count);

    let s = "æxtu einmæli, yggr var þeim síðany̆";
    let char_count = s.chars().count();
    println!("Character count: {}", char_count);
}

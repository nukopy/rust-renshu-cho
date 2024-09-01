use crate::args::Args;

#[allow(dead_code)]
fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

#[allow(dead_code)]
pub fn print_args(args: Args) {
    println!("{:#?}", args);
    println!("Type of args.files: {}", type_of(&args.files));
    println!("Type of args.lines: {}", type_of(&args.lines));
    println!("Type of args.bytes: {}", type_of(&args.bytes));
}

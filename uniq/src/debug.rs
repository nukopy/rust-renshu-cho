use crate::args::Args;

#[allow(dead_code)]
fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

#[allow(dead_code)]
pub fn print_args(args: Args) {
    println!("{:#?}", args);
}

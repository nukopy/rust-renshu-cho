// ---------------------------------------
// map
// ---------------------------------------

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`.
// Otherwise, return the peeled food.
// 食べ物の皮をむく。存在しない場合は単純に`None`を返す。
// そうでなければ皮を向いた食べ物を返す。
fn peel(food: Option<Food>) -> Option<Peeled> {
    #[allow(clippy::manual_map)]
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// Chopping food. If there isn't any, then return `None`.
// Otherwise, return the chopped food.
// 上と同じように、食べ物を切る前に、皮を向いた食べ物の有無を知る必要がある。
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    #[allow(clippy::manual_map)]
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// Cooking food. Here, we showcase `map()` instead of `match` for case handling.
// 上のチェックと同様だが`match`の代わりに`map()`を使用している。
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// A function to peel, chop, and cook food all in sequence.
// We chain multiple uses of `map()` to simplify the code.
// 複数の`map()`をチェインさせて、上のプロセスをシンプルにすることもできる。
fn process(food: Option<Food>) -> Option<Cooked> {
    #[allow(clippy::redundant_closure)]
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
// 食べる前に、食べ物の有無をチェックするのは大事ですよね!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    // よりシンプルな見た目の`process()`を使用する。
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

#[cfg(test)]
mod tests {
    // ---------------------------------------
    // map_or
    // ---------------------------------------

    #[test]
    fn map_or_practice() {
        let x = Some("foo");
        let x_len = x.map_or(42, |v| v.len());
        assert_eq!(x_len, 3);

        let x: Option<&str> = None;
        assert_eq!(x.map_or(42, |v| v.len()), 42);
    }

    // ---------------------------------------
    // map_or
    // ---------------------------------------

    fn expensive_calculation() -> i32 {
        println!("高コストな計算を実行中...");
        // 実際の高コストな計算をシミュレート
        std::thread::sleep(std::time::Duration::from_secs(2));
        42
    }

    #[test]
    fn map_or_else_practice() {
        fn main() {
            let maybe_value: Option<i32> = None;

            // map_or を使用した場合（即時評価）
            println!("map_or を使用:");
            let result1 = maybe_value.map_or(expensive_calculation(), |v| v * 2);
            println!("結果: {}", result1);

            println!("\nmap_or_else を使用:");
            // map_or_else を使用した場合（遅延評価）
            let result2 = maybe_value.map_or_else(expensive_calculation, |v| v * 2);
            println!("結果: {}", result2);

            // Some 値がある場合
            let maybe_value: Option<i32> = Some(10);

            println!("\nSome 値がある場合の map_or:");
            let result3 = maybe_value.map_or(expensive_calculation(), |v| v * 2);
            println!("結果: {}", result3);

            println!("\nSome 値がある場合の map_or_else:");
            let result4 = maybe_value.map_or_else(expensive_calculation, |v| v * 2);
            println!("結果: {}", result4);
        }
    }
}

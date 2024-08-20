use predicates::prelude::*;

fn main() {
    // 1. 個別の条件（述語）を作成する
    let is_positive = predicate::function(|&x: &i32| x > 0);
    let is_even = predicate::function(|&x: &i32| x % 2 == 0);
    let less_than_100 = predicate::lt(100);

    // 2. 条件を組み合わせて複雑な条件を構築する
    // 条件：正かつ偶数かつ 100 未満
    let complex_condition = is_positive.and(is_even).and(less_than_100);

    // 3. eval メソッドで入力の評価を行う
    println!(
        "24 satisfies the condition: {}",
        complex_condition.eval(&24)
    );
    println!(
        "50 satisfies the condition: {}",
        complex_condition.eval(&50)
    );
    println!(
        "-6 satisfies the condition: {}",
        complex_condition.eval(&-6)
    );
    println!(
        "101 satisfies the condition: {}",
        complex_condition.eval(&101)
    );

    // for ループで配列を使って評価する場合
    let values = [24, 50, -6, 101];
    for &value in &values {
        println!(
            "{} satisfies the condition: {}",
            value,
            complex_condition.eval(&value)
        );
    }
}

/* predicates::prelude::function
pub fn function<F, T>(function: F) -> FnPredicate<F>
where
    F: Fn(&T) -> bool,
    T: ?Sized,
*/

/*
 * use predicates::prelude::*;

fn main() {
    // 1. 個別の条件（述語）を作成
    let is_positive = predicate::function(|&x: &i32| x > 0);
    let is_even = predicate::function(|&x: &i32| x % 2 == 0);
    let less_than_100 = predicate::lt(100);

    // 2. 条件を組み合わせて複雑な条件を構築
    let complex_condition = is_positive
        .and(is_even)
        .and(less_than_100);

    // 3. evalメソッドで実際の評価を行う
    println!("24 satisfies the condition: {}", complex_condition.eval(&24));
    println!("50 satisfies the condition: {}", complex_condition.eval(&50));
    println!("-6 satisfies the condition: {}", complex_condition.eval(&-6));
    println!("101 satisfies the condition: {}", complex_condition.eval(&101));

    // 別の値で評価
    let values = [24, 50, -6, 101];
    for &value in &values {
        println!("{} satisfies the condition: {}", value, complex_condition.eval(&value));
    }
}
 */

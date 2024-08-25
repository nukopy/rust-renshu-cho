pub fn div(left: u64, right: u64) -> Result<u64, String> {
    // divison by zero
    if right == 0 {
        return Err("0 で割ることはできません".to_string());
    }

    // calc
    let div = left / right;

    Ok(div)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_10_by_2_is_5() {
        // given
        let left = 10;
        let right = 2;

        // when
        let result = div(left, right);

        // then
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn div_10_by_0_occur_error() {
        // given
        let left = 10;
        let right = 0;

        // when
        let result = div(left, right);

        // then
        assert!(result.is_err());
        assert_eq!(result, Err("0 で割ることはできません".to_string()));
    }
}

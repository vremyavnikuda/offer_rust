///
///
/// # Arguments
///
/// * `n`:
///
/// returns: bool
///
/// # Examples
///
/// ```
///
/// ```
pub(crate) fn is_square(n: i64) -> bool {
    if n < 0 {
        false // Отрицательные числа не являются квадратами
    } else {
        let sqrt_n = (n as f64).sqrt() as i64;
        sqrt_n * sqrt_n == n // Проверяем, равен ли квадрат корня n самому n
    }
}

#[cfg(test)]
mod tests {
    use super::is_square;

    #[test]
    fn fixed_tests() {
        assert_eq!(is_square(-1), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(0), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(3), false, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(4), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(25), true, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(is_square(26), false, "\nYour answer (left) is not the expected answer (right).");
    }
}
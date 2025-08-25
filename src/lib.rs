use crossterm::terminal::size;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ProgressBar {
    current: usize,
    total: usize,
    columns: usize,
}

impl ProgressBar {
    pub fn new<T>(total: T) -> Self
    where
        T: TryInto<usize>,
        T::Error: fmt::Debug,
    {
        let (t_columns, _) = size().unwrap();

        Self {
            current: 0,
            total: total.try_into().unwrap(),
            columns: t_columns.into(),
        }
    }
}

impl Iterator for ProgressBar {
    type Item = ProgressBar;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.total {
            println!();
            return None;
        }
        self.current += 1;
        let percent = (self.current as f64 / self.total as f64 * 100.0) as usize;

        let calc_message = format!(
            "  current: {:3}/{:3} {:3}% ",
            self.current, self.total, percent
        );

        let mut bar = String::new();

        let actual_columns = self.columns - calc_message.len() - 2;

        let actual_percent = (percent as f64 / 100.0) * actual_columns as f64;

        bar.push('[');
        for _ in 0..actual_percent as usize {
            bar.push('#');
        }
        for _ in actual_percent as usize..actual_columns {
            bar.push('.');
        }
        bar.push(']');

        let message = format!("\r{}{}", bar, calc_message);
        print!("{}", message);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        Some(self.clone())
    }
}

impl<A> FromIterator<A> for ProgressBar {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let count = iter.into_iter().count();
        Self::new(count as u64)
    }
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use crate::ProgressBar;

    #[test]
    fn test_hello_world() {
        assert!("hello world".to_string().len() > 0)
    }

    #[test]
    fn test_iter_one() {
        let mut p = ProgressBar::new(10);
        p.next();
        assert_eq!(p.current, 1)
    }
    #[test]
    fn test_iter_two() {
        let mut p = ProgressBar::new(10);
        p.next();
        p.next();
        assert_eq!(p.current, 2)
    }
    #[test]
    fn test_iter_final() {
        let mut p = ProgressBar::new(10);
        for _ in 0..10 {
            p.next();
        }
        assert_eq!(p.current, 10)
    }
    #[test]
    fn test_iter() {
        let p = ProgressBar::new(10);
        let mut count = 0;
        for _ in p {
            count += 1;
        }
        assert_eq!(count, 10)
    }

    #[test]
    fn test_from_iter_i32() {
        let iter: Range<i32> = (0..500).into_iter();
        let mut count = 0;
        let p = ProgressBar::from_iter(iter);
        for _ in p {
            count += 1;
        }
        assert_eq!(count, 500)
    }
}

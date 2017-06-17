use std::collections::BTreeMap;
use std::collections::Bound::*;

type Position = i64;

pub struct Curve<T> {
    points: BTreeMap<Position, T>
}

pub trait Interpolatable<T> {
    fn interpolate(&self, wanted_key: &Position, pre_key: &Position, post_key: &Position, pre_value: &T, post_value: &T) -> T;
}

impl Interpolatable<i64> for i64 {
    fn interpolate(&self, wanted_key: &Position, pre_key: &Position, post_key: &Position, pre_value: &i64, post_value: &i64) -> i64 {
        let alpha = (wanted_key - pre_key) as f64 / (post_key - pre_key) as f64;
        let pre_value_f = *pre_value as f64;
        let post_value_f = *post_value as f64;
        return ((1.0 - alpha) * pre_value_f + alpha * post_value_f) as i64;
    }
}

impl<T: Interpolatable<T> + Clone> Curve<T> {
    pub fn new() -> Curve<T> {
        Curve {
            points: BTreeMap::new()
        }
    }

    pub fn set(&mut self, key: Position, value: T) {
        self.points.insert(key, value);
    }

    pub fn value_at(&self, wanted_key: &Position) -> T {
        let mut pre_range = self.points.range((Unbounded, Included(wanted_key)));
        let (pre_key, pre_value) = pre_range.next_back().unwrap();

        if pre_key == wanted_key {
            return pre_value.clone();
        }

        let mut post_range = self.points.range((Included(wanted_key), Unbounded));
        let (post_key, post_value) = post_range.next().unwrap();

        return (*pre_value).interpolate(wanted_key, pre_key, post_key, pre_value, post_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c: Curve<i64> = Curve::new();
        c.set(1, 100);
        c.set(3, 300);
        c.set(6, 600);
        assert_eq!(c.value_at(&1), 100);
        assert_eq!(c.value_at(&3), 300);
        assert_eq!(c.value_at(&6), 600);

        assert_eq!(c.value_at(&2), 200);
        assert_eq!(c.value_at(&4), 400);
        assert_eq!(c.value_at(&5), 500);
    }
}
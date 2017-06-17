use std::collections::BTreeMap;
use std::collections::Bound::*;

type Position = i64;
type Keyframe = i64;

pub struct Curve {
    points: BTreeMap<Position, Keyframe>
}

impl Curve {
    pub fn new() -> Curve {
        Curve {
            points: BTreeMap::new()
        }
    }

    pub fn set(&mut self, key: Position, value: Keyframe) {
        self.points.insert(key, value);
    }

    pub fn value_at(&self, wanted_key: &Position) -> Keyframe {
        let mut pre_range = self.points.range((Unbounded, Included(wanted_key)));
        let (pre_key, pre_value) = pre_range.next_back().unwrap();

        if pre_key == wanted_key {
            return *pre_value;
        }

        let mut post_range = self.points.range((Included(wanted_key), Unbounded));
        let (post_key, post_value) = post_range.next().unwrap();

        let alpha = (wanted_key - pre_key) as f64 / (post_key - pre_key) as f64;
        let pre_value_f = *pre_value as f64;
        let post_value_f = *post_value as f64;
        return ((1.0 - alpha) * pre_value_f + alpha * post_value_f) as Keyframe;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = Curve::new();
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
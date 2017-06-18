use std::collections::BTreeMap;
use std::collections::Bound::*;

type Position = i64;
type Keyframe<'a, T> = (&'a Position, &'a T);

pub struct Curve<T> {
    points: BTreeMap<Position, T>
}

pub trait Interpolatable<'a, T> {
    fn interpolate(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, T>>, forward: &mut Iterator<Item=Keyframe<'a, T>>) -> T;
}

fn get_linear_interpolator<'a, T>(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, T>>, forward: &mut Iterator<Item=Keyframe<'a, T>>) -> (f64, &'a T, &'a T) {
    let pre = back.next().unwrap();

    if pre.0 == wanted_key {
        return (0.0, pre.1, pre.1);
    }

    let post = forward.next().unwrap();

    let alpha = (wanted_key - pre.0) as f64 / (post.0 - pre.0) as f64;
    return (alpha, pre.1, post.1);
}

impl<'a> Interpolatable<'a, i64> for i64 {
    fn interpolate(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, i64>>, forward: &mut Iterator<Item=Keyframe<'a, i64>>) -> i64 {
        let i = get_linear_interpolator(wanted_key, back, forward);
        return ((1.0 - i.0) * (*i.1 as f64) + i.0 * (*i.2 as f64)) as i64;
    }
    
}

impl<'a, T: Interpolatable<'a, T> + Clone> Curve<T> where T: Sized {
    pub fn new() -> Curve<T> {
        Curve {
            points: BTreeMap::new()
        }
    }

    pub fn set(&mut self, key: Position, value: T) {
        self.points.insert(key, value);
    }

    pub fn value_at(&'a self, wanted_key: &Position) -> T {
        let mut pre_range = self.points.range((Unbounded, Included(wanted_key)));
        let mut post_range = self.points.range((Included(wanted_key), Unbounded));
        
        T::interpolate(wanted_key, pre_range.by_ref(), post_range.by_ref())
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
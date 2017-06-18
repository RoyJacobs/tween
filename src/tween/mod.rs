use std::collections::BTreeMap;
use std::collections::Bound::*;
use std::marker::PhantomData;

type Position = i64;
type Keyframe<'a, T> = (&'a Position, &'a T);

pub struct Curve<T, IP> {
    points: BTreeMap<Position, T>,
    interpolator: PhantomData<IP>
}

pub trait Interpolator {
    fn get<'a, T>(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, T>>, forward: &mut Iterator<Item=Keyframe<'a, T>>) -> (f64, &'a T, &'a T);
}

pub struct LinearInterpolator {}
pub struct HoldInterpolator {}

impl Interpolator for LinearInterpolator {
    fn get<'a, T>(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, T>>, forward: &mut Iterator<Item=Keyframe<'a, T>>) -> (f64, &'a T, &'a T) {
        if let Some(post) = forward.next() {
            if wanted_key == post.0 {
                return (0.0, post.1, post.1)
            } else {
                let pre = back.next().unwrap();
                let alpha = (wanted_key - pre.0) as f64 / (post.0 - pre.0) as f64;
                return (alpha, pre.1, post.1);
            }
        }

        panic!("too far")
    }
}

impl Interpolator for HoldInterpolator {
    fn get<'a, T>(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, T>>, forward: &mut Iterator<Item=Keyframe<'a, T>>) -> (f64, &'a T, &'a T) {
        if let Some(post) = forward.next() {
            if wanted_key == post.0 {
                return (0.0, post.1, post.1)
            }
        }
        
        match back.next() {
            Some(pre) => (0.0, pre.1, pre.1),
            None => panic!("Too far")
        }
    }
}

pub trait Interpolatable<'a, T, IP> where IP: Interpolator {
    fn interpolate(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, T>>, forward: &mut Iterator<Item=Keyframe<'a, T>>) -> T;
}

impl<'a, IP> Interpolatable<'a, i64, IP> for i64 where IP: Interpolator {
    fn interpolate(wanted_key: &Position, back: &mut Iterator<Item=Keyframe<'a, i64>>, forward: &mut Iterator<Item=Keyframe<'a, i64>>) -> i64 {
        let i = IP::get(wanted_key, back, forward);
        return ((1.0 - i.0) * (*i.1 as f64) + i.0 * (*i.2 as f64)) as i64;
    }
}

impl<'a, T, IP> Curve<T, IP> where T: Sized + Interpolatable<'a, T, IP>, IP: Interpolator {
    pub fn new() -> Curve<T, IP> {
        Curve {
            points: BTreeMap::new(),
            interpolator: PhantomData,
        }
    }

    pub fn set(&mut self, key: Position, value: T) {
        self.points.insert(key, value);
    }

    pub fn value_at(&'a self, wanted_key: &Position) -> T {
        let pre_range = self.points.range((Unbounded, Excluded(wanted_key)));
        let mut post_range = self.points.range((Included(wanted_key), Unbounded));
        
        T::interpolate(wanted_key, pre_range.rev().by_ref(), post_range.by_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_interpolation_works() {
        let mut c: Curve<i64, LinearInterpolator> = Curve::new();
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

    #[test]
    fn hold_interpolation_works() {
        let mut c: Curve<i64, HoldInterpolator> = Curve::new();
        c.set(1, 100);
        c.set(3, 300);
        c.set(6, 600);
        assert_eq!(c.value_at(&1), 100);
        assert_eq!(c.value_at(&3), 300);
        assert_eq!(c.value_at(&6), 600);

        assert_eq!(c.value_at(&2), 100);
        assert_eq!(c.value_at(&4), 300);
        assert_eq!(c.value_at(&5), 300);
    }
}
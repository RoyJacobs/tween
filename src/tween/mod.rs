use std::collections::BTreeMap;
use std::collections::Bound::*;
use std::marker::PhantomData;

type Position = i64;
type Keyframe<'a, T> = (&'a Position, &'a T);

pub struct Curve<T, IP> {
    points: BTreeMap<Position, T>,
    interpolator: PhantomData<IP>
}

pub trait Interpolatable<'a, T> {
    fn scale(val: &T, amount: f64) -> T;
    fn add(x: T, y: T) -> T;
}

impl<'a> Interpolatable<'a, f64> for f64 {
    fn scale(val: &f64, amount: f64) -> f64 {
        *val * amount
    }

    fn add(x: f64, y: f64) -> f64 {
        x + y
    }
}

pub trait Interpolator {
    fn get<'a, T: Interpolatable<'a, T>>(time: f64, duration: f64, pre: &Keyframe<T>, post: &Keyframe<T>) -> T;
}

pub struct LinearInterpolator {}
pub struct HoldInterpolator {}

impl Interpolator for LinearInterpolator {
    fn get<'a, T: Interpolatable<'a, T>>(time: f64, duration: f64, pre: &Keyframe<T>, post: &Keyframe<T>) -> T {
        let alpha = time / duration;

        let p1 = T::scale(pre.1, 1.0 - alpha);
        let p2 = T::scale(post.1, alpha);

        return T::add(p1, p2);
    }
}

impl Interpolator for HoldInterpolator {
    fn get<'a, T: Interpolatable<'a, T>>(_: f64, _: f64, pre: &Keyframe<T>, _: &Keyframe<T>) -> T {
        return T::scale(pre.1, 1.0);
    }
}

impl<'a, T, IP> Curve<T, IP> where T: Clone + Interpolatable<'a, T>, IP: Interpolator {
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
        let mut post_range = self.points.range((Included(wanted_key), Unbounded));
        if let Some(post) = post_range.next() {
            if wanted_key == post.0 {
                return (*post.1).clone();
            }

            let mut pre_range = self.points.range((Unbounded, Excluded(wanted_key)));
            let pre = pre_range.next_back().unwrap();

            let time = (wanted_key - pre.0) as f64;
            let duration = (post.0 - pre.0) as f64;
            return IP::get(time, duration, &pre, &post);
        }

        panic!("too far");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_interpolation_works() {
        let mut c: Curve<f64, LinearInterpolator> = Curve::new();
        c.set(1, 100.0);
        c.set(3, 300.0);
        c.set(6, 600.0);
        assert_eq!(c.value_at(&1), 100.0);
        assert_eq!(c.value_at(&3), 300.0);
        assert_eq!(c.value_at(&6), 600.0);

        assert_eq!(c.value_at(&2), 200.0);
        assert_eq!(c.value_at(&4), 400.0);
        assert_eq!(c.value_at(&5), 500.0);
    }

    #[test]
    fn hold_interpolation_works() {
        let mut c: Curve<f64, HoldInterpolator> = Curve::new();
        c.set(1, 100.0);
        c.set(3, 300.0);
        c.set(6, 600.0);
        assert_eq!(c.value_at(&1), 100.0);
        assert_eq!(c.value_at(&3), 300.0);
        assert_eq!(c.value_at(&6), 600.0);

        assert_eq!(c.value_at(&2), 100.0);
        assert_eq!(c.value_at(&4), 300.0);
        assert_eq!(c.value_at(&5), 300.0);
    }
}
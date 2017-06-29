use std::collections::BTreeMap;
use std::collections::Bound::*;
use std::marker::PhantomData;

type Position = i64;
type Time = f64;
type Keyframe<'a, T> = (&'a Position, &'a T);

pub trait Curve<T> {
    fn set(&mut self, key: Position, value: T);
    fn value_at(&self, wanted_key: &Position) -> T;
}

pub trait Interpolatable<'a, T> {
    fn interpolate(pre: &Keyframe<T>, post: &Keyframe<T>, time: Time) -> T;
}

impl<'a> Interpolatable<'a, f64> for f64 {
    fn interpolate(pre: &Keyframe<f64>, post: &Keyframe<f64>, time: Time) -> f64 {
        if pre.0 == post.0 {
            return *pre.1;
        }

        let alpha = (time - (*pre.0 as Time)) / ((post.0 - pre.0) as Time);
        let p1 = pre.1 * (1.0 - alpha);
        let p2 = post.1 * alpha;
        return p1 + p2;
    }
}

pub trait Interpolator {
    fn get<'a, T: Interpolatable<'a, T>>(pre: &Keyframe<T>, post: &Keyframe<T>, time: Time) -> T;
}

pub struct LinearInterpolator;
pub struct HoldInterpolator;

impl Interpolator for LinearInterpolator {
    fn get<'a, T: Interpolatable<'a, T>>(pre: &Keyframe<T>, post: &Keyframe<T>, time: Time) -> T {
        return T::interpolate(pre, post, time);

    }
}

impl Interpolator for HoldInterpolator {
    fn get<'a, T: Interpolatable<'a, T>>(pre: &Keyframe<T>, _: &Keyframe<T>, _: Time) -> T {
        return T::interpolate(pre, pre, *pre.0 as Time);
    }
}

pub struct BTreeCurve<T, IP: Interpolator> {
    points: BTreeMap<Position, T>,
    interpolator: PhantomData<IP>
}

impl <'a, T, IP> BTreeCurve<T, IP> where T: Clone + Interpolatable<'a, T> + 'static, IP: Interpolator + 'static {
    pub fn new() -> Box<Curve<T>> {
        return Box::new(BTreeCurve::<T, IP> {
            points: BTreeMap::new(),
            interpolator: PhantomData
        });
    }
}

impl <'a, T, IP> Curve<T> for BTreeCurve<T, IP> where T: Clone + Interpolatable<'a, T>, IP: Interpolator {
    fn set(&mut self, key: Position, value: T) {
        self.points.insert(key, value);
    }

    fn value_at(&self, wanted_key: &Position) -> T {
        let mut post_range = self.points.range((Included(wanted_key), Unbounded));
        if let Some(post) = post_range.next() {
            if wanted_key == post.0 {
                return (*post.1).clone();
            }

            let mut pre_range = self.points.range((Unbounded, Excluded(wanted_key)));
            let pre = pre_range.next_back().unwrap();

            return IP::get(&pre, &post, *wanted_key as Time);
        }

        panic!("too far");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_interpolation_works() {
        let mut c = BTreeCurve::<f64, LinearInterpolator>::new();
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
        let mut c = BTreeCurve::<f64, HoldInterpolator>::new();
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
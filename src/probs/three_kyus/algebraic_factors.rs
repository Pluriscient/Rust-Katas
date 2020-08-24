#![allow(dead_code)]

use std::ptr::null;

// https://www.codewars.com/kata/5922543bf9c15705d0000020/train/rust
/// so, when are two type, `a` and `b`, considered equal?
/// a definition might be, it is possible to go from `a` to `b`,
/// and from `b` to `a`.
/// Going a roundway trip should leave you the same value.
/// Unfortunately it is virtually impossible to test this.
/// This is called ISO.
pub enum Void {}

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

pub type ISO<A, B> = (Box<dyn Fn(A) -> B>, Box<dyn Fn(B) -> A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
where
    F1: 'static + Fn(A) -> B,
    F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Box<dyn Fn(A) -> B> {
    iso.0
}

/// and vice versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Box<dyn Fn(B) -> A> {
    iso.1
}

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
    iso(|x| x, |x| x)
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    iso(|x: bool| !x, |x: bool| !x)
}

/// isomorphism is reflexive
pub fn refl<A: 'static>() -> ISO<A, A> {
    iso(|a: A| a, |a: A| a)
}

/// isomorphism is symmetric
pub fn symm<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<B, A> {
    iso(i.1, i.0)
}

/// isomorphism is transitive
pub fn trans<A: 'static, B: 'static, C: 'static>(ab: ISO<A, B>, bc: ISO<B, C>) -> ISO<A, C> {
    let (x_ab, x_bc) = (ab.0, bc.0);
    let (x_ba, x_cb) = (ab.1, bc.1);

    let left = move |a| x_bc(x_ab(a));
    let right = move |c| x_ba(x_cb(c));

    iso(left, right)
}

/// we can combine isomorphism
pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    let (x_ab, x_cd) = (ab.0, cd.0);
    let (x_ba, x_dc) = (ab.1, cd.1);
    let left = move |(a, c)| (x_ab(a), x_cd(c));
    let right = move |(b, d)| (x_ba(b), x_dc(d));
    iso(left, right)
}

pub fn iso_vec<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
    let (ab, ba) = (i.0, i.1);

    let left = move |xs: Vec<A>| xs.into_iter().map(&ab).collect();
    let right = move |xs: Vec<B>| xs.into_iter().map(&ba).collect();

    iso(left, right)
}

pub fn iso_option<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    let (ab, ba) = (i.0, i.1);

    let left = move |o_a| match o_a {
        None => None,
        Some(a) => Some(ab(a)),
    };
    let right = move |o_b| match o_b {
        None => None,
        Some(b) => Some(ba(b)),
    };
    iso(left, right)
}

pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    let (x_ab, x_cd) = (ab.0, cd.0);
    let (x_ba, x_dc) = (ab.1, cd.1);
    let left = move |o_ac| match o_ac {
        Ok(a) => Ok(x_ab(a)),
        Err(c) => Err(x_cd(c)),
    };
    let right = move |o_bd| match o_bd {
        Ok(b) => Ok(x_ba(b)),
        Err(d) => Err(x_dc(d)),
    };
    iso(left, right)
}

/// Going another way is hard (and is generally impossible)
/// Remember, for all valid ISO, converting and converting back
/// is the same as the original value.
/// You need this to prove some case are impossible.
pub fn iso_un_option<A: 'static, B: 'static>(i: ISO<Option<A>, Option<B>>) -> ISO<A, B> {
    let (ab, ba) = (i.0, i.1);
    let left = move |a| ab(Some(a)).unwrap();
    let right = move |b| ba(Some(b)).unwrap();
    iso(left, right)
}

/// inf + 0 = inf + 1
pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
    // iso(|a| Ok(Vec::new()), |b| Ok(Vec::new()))
    let left: Box<dyn Fn(Result<Vec<()>, ()>) -> Result<Vec<()>, Void>> =
        Box::new(|l: Result<Vec<()>, ()>| match l {
            Ok(l_content) => Ok(l_content),
            Err(()) => unreachable!(),
        });
    let right: Box<dyn Fn(Result<Vec<()>, Void>) -> Result<Vec<()>, ()>> =
        Box::new(|r: Result<Vec<()>, Void>| match r {
            Ok(r_content) => Ok(r_content),
            Err(_) => Err(()),
        });
    (left, right)
}

pub type IsoFL<A, B, C, D> = Box<dyn FnOnce(Box<dyn Fn(A) -> C>) -> Box<dyn FnOnce(B) -> D>>;
pub type IsoFR<A, B, C, D> = Box<dyn FnOnce(Box<dyn Fn(B) -> D>) -> Box<dyn FnOnce(A) -> C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

/// translator note:
/// FnBox is not yet supported, we can only return an uncallable
/// Box<FnOnce> (RetFunc). You should return the function with
/// correct type, which will be checked by the tests.
/// The type annotation is shown above. You need you return something like
/// (Box::new(...), Box::new(...))
pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    let (atob, ctod) = (ab.0, cd.0);
    let (btoa, dtoc) = (ab.1, cd.1);
    let left: IsoFL<A, B, C, D> =
        Box::new(move |atoc: Box<dyn Fn(A) -> C>| Box::new(move |b: B| ctod(atoc(btoa(b)))));
    let right: IsoFR<A, B, C, D> =
        Box::new(move |btod: Box<dyn Fn(B) -> D>| Box::new(move |a: A| dtoc(btod(atob(a)))));

    (left, right)
}

/// And we have isomorphism on isomorphism!
pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
    todo!()
    // // create an iso that converts
    // let a_to_b = iso();
    // let b_to_a = symm(a_to_b);
    // iso(a_to_b, b_to_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn lrl(iso: ISO<A, B>, start: A) {}
    //
    // fn test_inf() {
    //     assert_eq!(Ok(lu()), lrl(iso_eu(), Ok(lu())));
    // }

    #[test]
    fn test_1() {
        assert!(sub_st_l(iso_bool())(true));
    }

    #[test]
    fn test_2() {
        assert!(!sub_st_l(iso_bool())(false));
    }
    #[test]
    fn test_3() {
        assert!(sub_st_l(iso_bool_not())(false));
    }

    #[test]
    fn sub_st_l_test() {
        assert!(sub_st_l(iso_bool())(true));
        assert!(!sub_st_l(iso_bool())(false));
        assert!(sub_st_l(iso_bool_not())(false));
    }
}

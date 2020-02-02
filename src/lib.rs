#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bdd.rs"));


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        unsafe {
            bdd_init(1000,1000);
            bdd_setvarnum(3);

            let a = bdd_ithvar(0);
            let b = bdd_ithvar(1);
            let c = bdd_ithvar(2);

            let ab = bdd_addref( bdd_apply(a, b, bddop_and) );
            let abc = bdd_addref( bdd_apply(ab, c, bddop_or) );

            let bc = bdd_addref( bdd_apply(b, c, bddop_and) );

            // tests with three variables
            // abc
            // 000
            // 001 +
            // 010
            // 011 +
            // 100
            // 101 +
            // 110 *
            // 111 *

            let count = bdd_satcount(bddtrue);
            assert_eq!(count, (1 << 3) as f64);

            let count = bdd_satcount(bddfalse);
            assert_eq!(count, 0f64);

            let count = bdd_satcount(abc);
            assert_eq!(count, 5.0);

            let count = bdd_satcount(bc);
            assert_eq!(count, 2.0);

            // add a forth variable
            bdd_setvarnum(4);
            let d = bdd_ithvar(3);

            let nd = bdd_addref( bdd_not(d) );
            let abcd = bdd_addref( bdd_apply(abc, nd, bddop_and) );

            let count = bdd_satcount(abcd);
            assert_eq!(count, 5.0);

            let count = bdd_satcount(abc);
            assert_eq!(count, 10.0);

            let count = bdd_satcount(bc);
            assert_eq!(count, 4.0);

            bdd_delref(abcd);
            bdd_delref(abc);
            bdd_delref(nd);
            bdd_delref(ab);
            // etc.

            bdd_done();
        }
    }
}

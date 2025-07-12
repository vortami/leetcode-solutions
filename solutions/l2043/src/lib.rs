pub struct Bank {
    accounts: Vec<i64>,
}

impl Bank {
    pub const fn new(balance: Vec<i64>) -> Self {
        Self { accounts: balance }
    }

    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        // verfify that account 2 exists *before* withdrawing from account 1
        account2 as usize <= self.accounts.len() && self.withdraw(account1, money) && self.deposit(account2, money)
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        let Some(account) = self.accounts.get_mut((account - 1) as usize) else {
            return false;
        };
        *account += money;
        true
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let Some(account) = self.accounts.get_mut((account - 1) as usize) else {
            return false;
        };
        if *account >= money {
            *account -= money;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::MaybeUninit;
    use std::fmt::Debug;

    impl Debug for Bank {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            <Vec<_> as Debug>::fmt(&self.accounts, f)
        }
    }

    #[derive(Debug)]
    enum Argument {
        Value(i64),
        Array(Vec<i64>),
    }

    impl Argument {
        fn as_int(&self) -> i32 {
            self.as_long() as i32
        }

        #[track_caller]
        fn as_long(&self) -> i64 {
            if let &Self::Value(n) = self {
                n
            } else {
                panic!()
            }
        }

        #[track_caller]
        fn array(&self) -> &Vec<i64> {
            if let Self::Array(vec) = self {
                vec
            } else {
                panic!()
            }
        }
    }

    impl From<i64> for Argument {
        fn from(value: i64) -> Self {
            Self::Value(value)
        }
    }

    impl From<Vec<i64>> for Argument {
        fn from(value: Vec<i64>) -> Self {
            Self::Array(value.into())
        }
    }

    #[derive(PartialEq, Debug)]
    enum ReturnValue {
        Unit,
        Bool(bool),
    }

    impl From<()> for ReturnValue {
        fn from((): ()) -> Self {
            Self::Unit
        }
    }

    impl From<bool> for ReturnValue {
        fn from(value: bool) -> Self {
            Self::Bool(value)
        }
    }

    fn test_runner(fns: Vec<&str>, args: Vec<Vec<Argument>>, rv: Vec<ReturnValue>) {
        assert!(fns.len() == args.len() && fns.len() == rv.len());
        let mut obj = MaybeUninit::uninit();

        for n in 0..fns.len() {
            macro_rules! assert_eq {
                ($l:expr, $r:expr) => {
                    if $l != $r {
                        let bank = unsafe { obj.assume_init_ref() };
                        eprint!(
                            "error ocurred while performing function {n} ({}) with args {:?}\n\nBANK(len: {}): {}",
                            fns[n], args[n], bank.accounts.len(), bank.accounts[n as usize]
                        );
                        for n in 0..bank.accounts.len() {
                            if n % 10 == 0 {
                                eprint!("\n[{n:>3}]");
                            }
                            eprint!(" {:>5}", bank.accounts[n]);
                        }
                        panic!("{:?} != {:?}", $l, $r);
                    }
                };
            }

            let args = &args[n];
            let rv = &rv[n];
            match fns[n] {
                "Bank" => {
                    obj = MaybeUninit::new(Bank::new(args[0].array().clone()));
                }
                "transfer" => {
                    assert_eq!(
                        &ReturnValue::Bool(unsafe { obj.assume_init_mut() }.transfer(
                            args[0].as_int(),
                            args[1].as_int(),
                            args[2].as_long()
                        )),
                        rv
                    );
                }
                "deposit" => {
                    assert_eq!(
                        &ReturnValue::Bool(
                            unsafe { obj.assume_init_mut() }
                                .deposit(args[0].as_int(), args[1].as_long())
                        ),
                        rv
                    )
                }
                "withdraw" => assert_eq!(
                    &ReturnValue::Bool(
                        unsafe { obj.assume_init_mut() }
                            .withdraw(args[0].as_int(), args[1].as_long())
                    ),
                    rv
                ),
                other => panic!("{other} is not a valid function"),
            }
        }
    }

    macro_rules! test_arg {
        ([$($args:expr),*]) => {
            $crate::tests::Argument::Array(
                ::std::vec![
                    $(
                        $args,
                    )*
                ]
            )
        };
        ($arg:expr) => {
            $crate::tests::Argument::Value($arg)
        }
    }

    macro_rules! test_args {
        ([$($args:tt),*]) => {
            ::std::vec![
                $(
                    test_arg!($args),
                )*
            ]
        };
    }

    macro_rules! test_rv {
        (null) => {
            ReturnValue::Unit
        };
        (true) => {
            ReturnValue::Bool(true)
        };
        (false) => {
            ReturnValue::Bool(false)
        };
    }

    macro_rules! define_test {
        ($fname:ident [$($fnames:expr),*] [$($args:tt),*] [$($rv:tt),*]) => {
            #[test]
            fn $fname() {
                $crate::tests::test_runner(
                    ::std::vec![$($fnames,)*],
                    ::std::vec![
                        $(
                            test_args!($args)
                        ),*
                    ],
                    ::std::vec![
                        $(
                            test_rv!($rv),
                        )*
                    ],
                );
            }
        };
    }

    define_test! {
        test1
        ["Bank","withdraw","transfer","deposit","transfer","withdraw"]
        [[[10,100,20,50,30]],[3,10],[5,1,20],[5,20],[3,4,15],[10,50]]
        [null,true,true,true,false,false]
    }

    define_test! {
        test2
        ["Bank","withdraw","transfer","transfer","withdraw","withdraw","transfer","deposit","deposit","transfer","transfer","deposit","withdraw","withdraw","withdraw","deposit","transfer","transfer","withdraw","transfer","withdraw","withdraw","withdraw","transfer","withdraw","withdraw","withdraw","transfer","withdraw","withdraw","transfer","deposit","transfer","transfer","deposit","transfer","transfer","transfer","transfer","transfer","transfer","deposit","transfer","withdraw","withdraw","transfer","deposit","withdraw","withdraw","withdraw","transfer","transfer","transfer","deposit","deposit","transfer","deposit","transfer","deposit","withdraw","deposit","deposit","deposit","deposit","transfer","withdraw","deposit","transfer","deposit","withdraw","deposit","withdraw","deposit","deposit","transfer","withdraw","transfer","transfer","withdraw","transfer","withdraw","transfer","deposit","transfer","withdraw","transfer","withdraw","deposit","withdraw","transfer","transfer","withdraw","withdraw","deposit","withdraw","transfer","withdraw","deposit","deposit","deposit","transfer","transfer","transfer","deposit","transfer","withdraw","withdraw","transfer","transfer","deposit","deposit","withdraw","transfer","deposit","deposit","transfer","deposit","deposit","deposit","deposit","withdraw","withdraw","transfer","deposit","transfer","withdraw","deposit","transfer","deposit","withdraw","transfer","withdraw","withdraw","deposit","deposit","transfer","transfer","deposit","withdraw","transfer","withdraw","withdraw","withdraw","transfer","withdraw","deposit","transfer","transfer","withdraw","deposit","deposit","withdraw","deposit","deposit","deposit","deposit","deposit"]
        [[[579,1143,1635,2634,5482,3297,2929,6276,5675,7527,5098,2942,5021,6868,2042,861,1820,7279,1119,8892,6404,3970,2381,8297,4816,4438,7661,8953,5095,3056,778,8597,4212,4963,6180,7206,7078,1277,5274,4951,8258,6131,4255,8297,8971,2959,4404,3890,689,9703,7300,8793,7960,3633,3112,3165,7050,3288,6162,9963,8585,4726,3176,7475,7227,7434,1384,1382,660,6489,1271,8640,4078,7255,9654,9369,6794,6229,7071,1385,7125,8458,3550,592,68,562,7312,7906,4098,4855,1931,1549,5464,7334,5875,5730,2232,1775,8297,8429,7520,6505,7638,6242,3005,6369,5981,870,9937,1056,2042,8185,1806,5585,2096,4233,9198,2549,9070,7604,64,3415,4289,7863,9015,9781,1433,4041,4942,5870,1900,2297,3473,8532,2392,9287,4968,9445,8735,6616,1702,3094,2638,512,1113,1077,5595,8787,221,4147,4877,7851,3267,5194,2150,7845,6253,9183,266,739,1017,8976,869,3638,588,150,6376,8318,1349,4944,1258,3987,5614,4984,4660,8996,9110,4522,6182,3122,4689,6609,3223,9604,2122,5132,1480,7423,6388,1816,3154,2249,6004,6200,6912,777,7846,5966,8576,5559,5648,5314,5960,669,6748,2316,3120,6319,1857,5999,5387,786,7063,3894,6986,2341,1830,6965,3690,2290,8256,4833,1760,5205,2306,1867,2371,5528,6407,6538,4614,2277,3274,6144,7765,5537,8428,2536,6666,5619,226,2877,4592,2252,4855,5394,4580,5916,9990,5685,2375,5836,4726,9866,8692,9112,8286,1216,3286,2260,8379,9402,1322,6215,1356,4029,362,8054,1060,3991,7627,6379,6420,6904,8198,9224,2035,2058,7833,5993,5266,2469,6890,8719,9771,3843,1810,280,1930,9350,1866,6500,5944,396,4460,8483,6272,1554,8695,6308,4480,6000,939,3633,1551,6465,8258,2484,4578,4219,4627,312,9571,6208,154,6852,7278,9652,6416,1466,5651,8157,4730,113,2360,3467,1163,5126,5704,8373,9182,8434,3737,8720,1848,4690,8738,7741,3251,3417,2618,2105,7959,831,7460,7379,2842,3431,4756,6682,5532,8583,1318,2975,8314,4848,907,1569,1949,6728,5278,3098,1327,5994,3092,2025,8196,117,588,5002,9958,8894,3027,2043,6268,9415,5804,9218,8538,9292,4706,3266,9947,3124,5114,4286,1165,5333,9966,4105,1308,2609,7902,3975,9660,5442,4902,4091,6107,5872,8439,7119,6080,8941,860,5524,3875,8600,3664,8427,8379,6476,150,296]],[2421,7151],[151,430,2869],[135,233,715],[1,7889],[75,797],[145,101,8692],[124,8964],[18,1334],[140,287,9090],[161,384,1008],[188,9799],[211,1195],[104,8497],[2948,4080],[129,3741],[321,171,3091],[107,228,1793],[190,701],[332,44,3946],[157,8842],[379,6085],[35,6819],[412,208,309],[2206,9469],[222,1407],[58,1652],[3812,183,1631],[1179,3897],[217,7195],[322,255,4812],[13,3028],[360,322,4090],[41,2975,3798],[117,6158],[116,406,7600],[133,249,4473],[358,307,5807],[217,173,2543],[2,221,5791],[111,204,1454],[219,4829],[179,69,9657],[217,2709],[111,5986],[27,186,9549],[37,4698],[288,2307],[238,6741],[211,6991],[405,64,1483],[1772,222,8026],[192,242,7974],[223,6953],[226,8329],[144,313,2477],[21,1618],[175,68,561],[109,8932],[117,8759],[371,8321],[99,5799],[237,5727],[581,557],[206,161,7402],[111,9363],[371,8775],[365,61,2158],[27,6235],[431,2367],[378,3074],[40,8175],[32,9029],[234,9786],[228,342,4565],[265,2902],[27,241,487],[267,288,8818],[117,6451],[388,279,6601],[349,3789],[283,230,6163],[272,3961],[2958,391,1567],[218,1147],[78,3839,5974],[42,5504],[394,8252],[1365,6333],[87,215,9328],[284,335,4256],[367,2575],[177,5931],[3,8089],[302,6595],[315,361,9208],[3601,7531],[297,782],[767,4355],[104,5557],[227,2681,2369],[149,249,1675],[347,328,1345],[63,7658],[396,297,4770],[386,1448],[2349,7436],[401,2914,4072],[375,2267,9574],[30,3361],[159,935],[209,4745],[3506,362,6961],[368,8707],[327,9819],[277,56,4967],[424,4686],[14,6890],[2567,2857],[286,8185],[307,3484],[151,4524],[206,176,8600],[107,2678],[180,362,9345],[392,6982],[338,6612],[149,30,4531],[299,8293],[157,3055],[402,82,6902],[2363,675],[752,2698],[275,9801],[68,9172],[282,145,7123],[339,213,4217],[182,1334],[43,2310],[29,278,5031],[1511,8965],[1590,250],[1631,7151],[2044,306,1438],[136,4068],[206,4672],[277,62,7349],[294,219,9601],[311,4784],[212,1881],[2638,4787],[3388,5228],[55,3931],[256,9470],[129,9951],[293,4834],[114,6020]]
        [null,false,false,true,false,true,false,true,true,false,true,true,true,false,false,true,true,true,true,true,false,true,false,true,false,true,true,false,false,false,true,true,true,false,true,false,false,false,false,false,true,true,false,false,false,false,true,false,false,false,false,false,false,true,true,false,true,true,true,true,true,true,true,false,false,false,true,true,true,false,true,false,true,true,true,false,true,false,true,false,true,true,true,false,true,false,true,true,false,false,true,true,true,true,false,false,false,true,false,true,false,false,true,true,true,true,false,false,false,true,true,false,false,true,true,false,false,true,false,true,true,true,false,true,false,false,true,false,true,true,true,false,false,true,true,false,false,true,true,true,false,false,false,false,true,true,false,false,false,true,false,false,true,true,true,true,true]
    }
}

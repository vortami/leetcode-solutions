pub struct ATM {
    notes: [i32; 5],
}

impl ATM {
    pub const fn new() -> Self {
        Self { notes: [0; 5] }
    }

    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        assert_eq!(banknotes_count.len(), 5);
        self.notes
            .iter_mut()
            .zip(banknotes_count)
            .for_each(|(l, r)| *l += r);
    }

    pub fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        const NOTE_VALUES: [i32; 5] = [20, 50, 100, 200, 500];
        let mut out = vec![0; NOTE_VALUES.len()];

        for idx in (0..NOTE_VALUES.len()).rev() {
            while amount >= NOTE_VALUES[idx] && self.notes[idx] > out[idx] {
                amount -= NOTE_VALUES[idx];
                out[idx] += 1;
            }
        }

        if amount != 0 {
            out.truncate(1);
            out[0] = -1;
        } else {
            for (idx, &amt) in out.iter().enumerate() {
                self.notes[idx] -= amt;
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::MaybeUninit;

    #[derive(Debug, PartialEq)]
    enum Argument {
        Unit,
        Number(i32),
        Array(Vec<i32>),
    }

    impl Argument {
        const fn as_int(&self) -> i32 {
            if let &Self::Number(n) = self {
                n
            } else {
                panic!()
            }
        }

        const fn as_array(&self) -> &Vec<i32> {
            if let Self::Array(vec) = self {
                vec
            } else {
                panic!()
            }
        }
    }

    fn test_runner(fns: Vec<&str>, args: Vec<Vec<Argument>>, rv: Vec<Argument>) {
        assert!(fns.len() == args.len() && fns.len() == rv.len());
        let mut obj = MaybeUninit::uninit();

        for n in 0..fns.len() {
            macro_rules! assert_eq {
                ($l:expr, $r:expr) => {
                    if $l != $r {
                        let atm = unsafe { obj.assume_init_ref() };
                        eprint!(
                            "error ocurred while performing function {n} ({}) with args {:?}\n\nATM: {:?}",
                            fns[n], args[n], atm.notes,
                        );
                        panic!("{:?} != {:?}", $l, $r);
                    }
                };
            }

            let args = &args[n];
            let rv = &rv[n];
            match fns[n] {
                "ATM" => {
                    obj = MaybeUninit::new(ATM::new());
                }
                "deposit" => unsafe { obj.assume_init_mut() }.deposit(args[0].as_array().clone()),
                "withdraw" => assert_eq!(
                    &Argument::Array(
                        unsafe { obj.assume_init_mut() }.withdraw(args[0].as_int())
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
            $crate::tests::Argument::Number($arg)
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
            Argument::Unit
        };
        ($other:tt) => {
            test_arg!($other)
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
        ["ATM","deposit","withdraw","deposit","withdraw","withdraw"]
        [[],[[0,0,1,2,1]],[600],[[0,1,0,1,1]],[600],[550]]
        [null,null,[0,0,1,0,1],null,[-1],[0,1,0,0,1]]
    }

    define_test! {
        test2
        ["ATM","withdraw","deposit","deposit","deposit","deposit","deposit","deposit","withdraw","deposit","withdraw","withdraw","withdraw","withdraw","withdraw","withdraw","withdraw","withdraw","withdraw","deposit","deposit","withdraw","deposit","deposit","withdraw","deposit","deposit","deposit","deposit","deposit","withdraw","withdraw","withdraw","deposit","withdraw","withdraw","deposit","deposit","deposit","deposit","withdraw","deposit","withdraw","withdraw","deposit","deposit","withdraw","withdraw","withdraw","withdraw","withdraw","withdraw","deposit","withdraw","withdraw","deposit","withdraw","withdraw","deposit","withdraw","withdraw","deposit","withdraw","deposit","withdraw","withdraw","deposit","deposit","withdraw","deposit","withdraw","withdraw","withdraw","deposit","deposit","withdraw","withdraw","withdraw","deposit","deposit","deposit","withdraw","withdraw","withdraw","withdraw","withdraw","deposit","deposit","withdraw","withdraw","deposit"]
        [[],[612519790],[[335778,848154,119256,88284,800761]],[[838123,734850,938357,767867,619568]],[[295545,644807,945715,235366,389410]],[[371938,643156,650866,276264,967618]],[[664440,931186,736602,608390,986995]],[[835266,327755,792986,248191,29300]],[272106650],[[427123,97369,944260,798500,523262]],[774276375],[97537035],[996467815],[641646035],[875306040],[527583655],[4020640],[205858665],[282796115],[[114141,872629,383737,520833,134076]],[[868438,293086,477931,358591,241162]],[186734200],[[257230,646928,76911,259434,442424]],[[868845,488537,420022,657569,229281]],[427402275],[[654675,636386,728985,136331,309723]],[[289998,210084,284156,264536,415559]],[[513953,975642,105349,768092,719161]],[[625716,605953,781441,363032,710089]],[[933718,411763,28886,606502,935793]],[903776210],[385312930],[147464610],[[880020,174924,768543,545233,915409]],[27188970],[927664515],[[90631,935661,920083,622910,833297]],[[336752,436746,894142,508751,54763]],[[400048,110441,629753,277098,441365]],[[240200,5471,344482,247972,755320]],[230497095],[[343447,405697,681062,693906,987571]],[875198145],[556259265],[[374368,385647,110593,667881,665376]],[[887209,823362,63784,779450,266752]],[868737680],[144845275],[383508580],[921012670],[298579885],[133180555],[[910314,607031,165,617788,138227]],[412948515],[439564120],[[128208,659666,452184,531386,6854]],[109540555],[495037185],[[612207,51305,18775,896288,695225]],[932540805],[467128975],[[80895,47225,536813,299239,605811]],[635988090],[[986063,307528,346819,437167,378711]],[220985165],[953573365],[[966153,395914,115817,71638,88534]],[[687027,594310,614797,134300,236869]],[418857735],[[306860,403144,553921,19892,194931]],[820107540],[640290840],[370152015],[[118984,931934,574313,35776,328736]],[[718988,125697,6891,679455,310540]],[263430160],[894356060],[693980375],[[706554,601584,207444,280311,569540]],[[626373,377287,326317,131202,270873]],[[376234,40174,576055,8190,867436]],[898399225],[334221705],[261678275],[67896200],[74844865],[[808395,751993,256915,407750,218915]],[[861632,761170,835281,512968,112263]],[244505],[702718995],[[473969,29870,586981,525277,55234]]]
        [null,[-1],null,null,null,null,null,null,[0,1,1,0,544213],null,[-1],[-1],[-1],[-1],[2,0,0,0,1750612],[-1],[2,0,1,0,8041],[-1],[-1],null,null,[0,0,0,1,373468],null,null,[-1],null,null,null,null,null,[-1],[-1],[-1],null,[1,1,0,2,54377],[-1],null,null,null,null,[-1],null,[-1],[-1],null,null,[-1],[-1],[-1],[1,1,1,0,1842025],[-1],[-1],null,[-1],[1,0,1,0,879128],null,[-1],[-1],null,[-1],[-1],null,[2,1,0,0,1271976],null,[-1],[-1],null,null,[-1],null,[2,0,0,0,1640215],[2,0,1,1,1280581],[-1],null,null,[-1],[-1],[-1],null,null,null,[-1],[-1],[-1],[0,0,0,1,135792],[-1],null,null,[-1],[-1],null]    }
}

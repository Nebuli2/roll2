use rand::prelude::*;
use std::env;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Exclude {
    Low,
    High,
    None,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Roll {
    num: u32,
    die: u32,
    bonus: i32,
    exclude: Exclude,
}

type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

/// Generates a random number in the range specified by the die.
fn roll_die(die: u32, mut rng: impl Rng) -> u32 {
    match die {
        0 => 0,
        n => rng.gen_range(1, n + 1),
    }
}

impl Roll {
    /// Simulates the `Roll`.
    fn roll(&self, mut rng: impl Rng) -> Option<i32> {
        print!("[");
        let printed_die = if self.die != 0 {
            if self.num != 1 {
                print!("{}", self.num);
            }
            print!("d{}", self.die);
            true
        } else {
            false
        };
        if self.bonus != 0 {
            if printed_die {
                print!("{:+}", self.bonus);
            } else {
                print!("{}", self.bonus);
            }
        }
        match self.exclude {
            Exclude::Low => print!(" - low"),
            Exclude::High => print!(" - high"),
            Exclude::None => {}
        }
        print!("] ");
        match self.num {
            1 => {
                let roll = roll_die(self.die, rng) as i32 + self.bonus;
                println!("{}", roll);
                Some(roll)
            }
            n if n > 1 => {
                let mut rolls = vec![];
                let mut exclude = match self.exclude {
                    Exclude::Low => std::u32::MAX,
                    Exclude::High => 0,
                    Exclude::None => 0,
                };
                for _ in 0..n {
                    let roll = roll_die(self.die, &mut rng);
                    match self.exclude {
                        Exclude::Low => {
                            if exclude > roll {
                                exclude = roll;
                            }
                        }
                        Exclude::High => {
                            if exclude < roll {
                                exclude = roll;
                            }
                        }
                        Exclude::None => {}
                    }
                    rolls.push(roll);
                }
                let mut excluded_shown = false;

                for &roll in rolls.iter() {
                    if roll == exclude && !excluded_shown {
                        excluded_shown = true;
                        print!("({}), ", roll);
                    } else {
                        print!("{}, ", roll);
                    }
                }

                let total = rolls.iter().sum::<u32>() as i32 + self.bonus;
                let exclude = exclude as i32;
                println!("final = {}", total - exclude);
                Some(total - exclude)
            }
            _ => {
                println!("no dice rolled");
                None
            }
        }
    }
}

/// Attempts to parse the specified argument. An argument is expected in the
/// form of either <number>d<die> or d<die>. In the latter case, the number of
/// dice is inferred to be 1. If the argument cannot be parsed, `None` is
/// returned instead.
fn parse_arg(arg: &str) -> Result<Roll> {
    const ERR_NO_DIE: &'static str = "invalid roll format: no die specified";
    const ERR_MOD_FMT: &'static str = "invalid roll format: modifier must be an integer";
    const ERR_DIE_FMT: &'static str = "invalid roll format: die must be an integer";

    if let Some(idx) = arg.find('d') {
        let (num, die) = arg.split_at(idx);
        let num: u32 = num.parse().unwrap_or_else(|_| 1);
        let die = die.trim_start_matches("d");

        // Check if we have a flat bonus
        let (die, bonus) = if let Some(bonus_idx) = die.find('+') {
            let (die, bonus) = die.split_at(bonus_idx);
            let bonus: i32 = bonus.parse().map_err(|_| ERR_MOD_FMT)?;
            let die: u32 = die.parse().map_err(|_| ERR_DIE_FMT)?;
            (die, bonus)
        } else if let Some(bonus_idx) = die.find('-') {
            let (die, bonus) = die.split_at(bonus_idx);
            let bonus: i32 = bonus.parse().map_err(|_| ERR_MOD_FMT)?;
            let die: u32 = die.parse().map_err(|_| ERR_DIE_FMT)?;
            (die, bonus)
        } else {
            let die: u32 = die.parse().map_err(|_| ERR_DIE_FMT)?;
            (die, 0)
        };

        Ok(Roll {
            num,
            die,
            bonus,
            exclude: Exclude::None,
        })
    } else {
        let flat_bonus: i32 = arg.parse().map_err(|_| ERR_NO_DIE)?;
        Ok(Roll {
            num: 1,
            die: 0,
            bonus: flat_bonus,
            exclude: Exclude::None,
        })
    }
}

trait ParseArgs {
    /// Attempts to parse the arguments into an `impl Iterator<Item=Roll`.
    fn parse_args(self) -> Result<Vec<Roll>>;
}

impl<T> ParseArgs for T
where
    T: Iterator<Item = String>,
{
    fn parse_args(self) -> Result<Vec<Roll>> {
        self.flat_map(|arg| match arg.as_str() {
            "adv" | "advantage" => vec![Ok(Roll {
                num: 2,
                die: 20,
                bonus: 0,
                exclude: Exclude::Low,
            })],
            "dis" | "disadvantage" => vec![Ok(Roll {
                num: 2,
                die: 20,
                bonus: 0,
                exclude: Exclude::High,
            })],
            "chaos" | "chaos_bolt" => vec![
                Ok(Roll {
                    num: 2,
                    die: 8,
                    bonus: 0,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::None,
                }),
            ],
            "stats" | "char" | "character" => vec![
                Ok(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Ok(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Ok(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Ok(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Ok(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
                Ok(Roll {
                    num: 4,
                    die: 6,
                    bonus: 0,
                    exclude: Exclude::Low,
                }),
            ],
            "tiny-objects" | "tiny" | "animate-objects" => vec![
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
                Ok(Roll {
                    num: 1,
                    die: 20,
                    bonus: 8,
                    exclude: Exclude::None,
                }),
            ],
            arg => vec![parse_arg(arg)],
        })
        .collect()
    }
}

fn main() {
    let name = env::args().next().unwrap();
    let rolls = env::args().skip(1).parse_args();
    match rolls {
        Ok(rolls) => match &rolls[..] {
            [] => println!("{}: no dice specified", name),
            [roll] => {
                let mut rng = thread_rng();
                roll.roll(&mut rng);
            }
            rolls => {
                let mut total = 0;
                let mut rng = thread_rng();
                for roll in rolls {
                    match roll.roll(&mut rng) {
                        Some(num) => {
                            total += num;
                        }
                        None => eprintln!("error has occurred"),
                    }
                }
                println!("Total roll: {}", total);
            }
        },
        Err(why) => println!("{}: {}", name, why),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_single() {
//         let parsed = parse_arg("4d6");
//         let expected = Some(Roll { num: 4, die: 6 });
//         assert_eq!(parsed, expected);

//         let parsed = parse_arg("d10");
//         let expected = Some(Roll { num: 1, die: 10 });
//         assert_eq!(parsed, expected);

//         let parsed = parse_arg("5");
//         let expected = None;
//         assert_eq!(parsed, expected);
//     }

//     #[test]
//     fn parse_multiple() {
//         let args = ["adv", "dis", "chaos", "4d6", "d20"]
//             .into_iter()
//             .map(ToString::to_string);
//         let parsed: Vec<_> = parse_args(args).collect();
//         let expected = vec![
//             Roll { num: 1, die: 20 },
//             Roll { num: 1, die: 20 },
//             Roll { num: 1, die: 20 },
//             Roll { num: 1, die: 20 },
//             Roll { num: 2, die: 8 },
//             Roll { num: 1, die: 6 },
//             Roll { num: 4, die: 6 },
//             Roll { num: 1, die: 20 },
//         ];
//         assert_eq!(parsed, expected);
//     }
// }

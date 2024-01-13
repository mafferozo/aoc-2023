mod args;

use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = args::get_input(2023, 20)?;

    part_one(&input);
    part_two(&input);
    Ok(())
}

fn part_one(input: &str) {
    let mut state = State::from_input(&input);
    let (low_count, high_count) = (&mut 0, &mut 0);

    // we simply brute force
    for _ in 0..1000 {
        state.broadcast_low(low_count, high_count);
    }

    println!("{}", *low_count * *high_count);
}

fn part_two(input: &str) {
    let mut state = State::from_input(&input);
    let (a,b,c,d) = state.presses_until_rx();
    let lcm = [a,b,c,d].into_iter().reduce(lcm).unwrap();
    println!("{}",lcm)

}

const PULSE_HIGH: u64 = 1;
const PULSE_LOW: u64 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Module {
    /// Module state.
    ///
    /// FlipFlop:
    /// 0u64 (off) or 1u64 (on)
    ///
    /// Conjunction:
    /// 0bx_0..x_n, where x is 0 or 1 for input at index x_0 up to x_n
    /// where n = the number of modules
    ///
    /// for the range x_n..x_63, all bits should be 1
    ///
    /// The 64th bit is a tag: if its 0, this state corresponds to a FlipFlop
    ///                        if its 1, this state corresponds to a Conjunction
    state: u64,
}

impl Module {
    // for now same as flipflop()
    // but use this if you don't care about the state
    fn new() -> Self {
        Self { state: 0 }
    }

    fn conjunction() -> Self {
        // initially, every input for a conjunction is 1
        Self { state: u64::MAX }
    }

    fn flipflop() -> Self {
        Self { state: 0 }
    }

    fn is_flipflop(&self) -> bool {
        (self.state >> 63) == 0
    }

    fn update_input(&mut self, index: usize, pulse: u64) {
        // assert!(self.is_conjunction());
        // assert!(index < 63);
        // assert!(pulse == PULSE_HIGH || pulse == PULSE_LOW);
        let mask = 1 << index;
        if pulse == PULSE_HIGH {
            // set bit
            self.state |= mask;
        } else {
            // unset bit
            self.state &= !mask;
        };
    }

    /// Process the next pulse, given:
    /// - where it came from (from)
    /// - an input pulse (in)
    ///
    /// Conjunctions always send a pulse,
    /// FlipFlops might send a pulse
    fn next(&mut self, from: usize, pulse: u64) -> Option<u64> {
        // assert!(pulse == PULSE_HIGH || pulse == PULSE_LOW);
        if self.is_flipflop() {
            if pulse == PULSE_LOW {
                // toggle state
                self.state ^= 1;
                Some(self.state)
            } else {
                None
            }
        } else {
            self.update_input(from, pulse);
            // if state only contains 1 bits, send 0 else 1
            // println!("{:#064b}",self.state);
            Some(if self.state == u64::MAX {
                PULSE_LOW
            } else {
                PULSE_HIGH
            })
        }
    }
}

#[derive(Debug)]
struct State {
    /// Maps names of the modules to an index in `mods`
    names: HashMap<String, usize>,

    /// Keeps track of the destinations in Modules for each module
    destinations: HashMap<usize, Vec<usize>>,

    /// A list of indices the broadcaster module broadcasts too
    broadcaster: Vec<usize>,

    /// The array of modules
    ///
    /// Due to the way module is implemented, we can only allocate a maximum of 63 modules
    mods: [Module; 63],

    /// Keeps track of the number of modules in `mods`
    len: usize,
}

impl State {
    fn new() -> Self {
        Self {
            names: HashMap::new(),
            destinations: HashMap::new(),
            broadcaster: Vec::new(),
            mods: [Module::new(); 63],
            len: 0,
        }
    }

    fn from_input(input: &str) -> Self {
        let tuples: Vec<(&str, &str, Vec<&str>)> = input
            .lines()
            .map(|l| {
                let first_char = &l[0..1];
                let key = if first_char == "b" {
                    &l[0..11]
                } else {
                    &l[1..3]
                };
                let destinations = if &l[0..1] == "b" { &l[15..] } else { &l[7..] };
                let destinations = destinations.split(", ").collect();
                (first_char, key, destinations)
            })
            .collect();

        let mut state = Self::new();

        // first add all modules
        for (first_char, key, _dests) in tuples.iter() {
            if *first_char == "b" {
                continue;
            }
            let module = if *first_char == "&" {
                Module::conjunction()
            } else {
                Module::flipflop()
            };
            state.push(module, key);
        }

        // then add all destinations,
        // and set inputs for conjunctions
        for (first_char, key, dests) in tuples.iter() {
            let indices = dests
                .iter()
                // map destinations that are not a module (rx!) to 64
                .map(|dest| state.get_index(*dest).unwrap_or(&64))
                .copied()
                .collect();
            if *first_char == "b" {
                state.broadcaster = indices;
                continue;
            }

            let key_index = *state.get_index(key).unwrap();
            state.destinations.insert(key_index, indices.clone());

            for dest_index in indices {
                let Some(module) = state.get_mut(dest_index) else {
                    continue;
                };
                if module.is_flipflop() {
                    continue;
                }
                // add key_index as input for conjunction at dest_index
                module.update_input(key_index, PULSE_LOW)
            }
        }
        state
    }

    fn push(&mut self, module: Module, key: &str) {
        // assert!(self.len < 63);

        // update modules
        let index = self.len;
        self.mods[index] = module;
        self.len += 1;

        // update names
        self.names.insert(key.into(), index);
    }

    fn get_index(&self, key: &str) -> Option<&usize> {
        self.names.get(key)
    }

    fn get_destinations(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        // assert!(index < self.len);
        self.destinations.get(&index).unwrap().iter().copied()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Module> {
        self.mods.get_mut(index)
    }

    fn broadcast_low(&mut self, low_count: &mut u64, high_count: &mut u64) {
        // add 1 to low_count for button press
        *low_count += 1;

        // queue up initial broadcast pulses
        let mut queue = VecDeque::from_iter(
            self.broadcaster
                .iter()
                .copied()
                .map(|to| (65, to, PULSE_LOW)),
        );

        while let Some((from, to, pulse)) = queue.pop_front() {
            // count pulse
            let c = if pulse == PULSE_HIGH {
                &mut *high_count
            } else {
                &mut *low_count
            };
            *c += 1;

            let Some(module) = self.get_mut(to) else {
                continue;
            };
            let Some(next_pulse) = module.next(from, pulse) else {
                continue;
            };

            for dest in self.get_destinations(to) {
                queue.push_back((to, dest, next_pulse))
            }
        }
    }

    fn presses_until_rx(&mut self) -> (usize, usize, usize, usize) {
        let mut presses = (usize::MAX, usize::MAX, usize::MAX, usize::MAX);
        let mut c = 0;

        let v = (
            self.names.get("lh").copied().unwrap(),
            self.names.get("fk").copied().unwrap(),
            self.names.get("ff").copied().unwrap(),
            self.names.get("mm").copied().unwrap(),
        );

        loop {
            c += 1;
            // queue up broadcast pulses from button press
            let mut queue = VecDeque::from_iter(
                self.broadcaster
                    .iter()
                    .copied()
                    .map(|to| (65, to, PULSE_LOW)),
            );

            while let Some((from, to, pulse)) = queue.pop_front() {
                // if to == v.0 && pulse == PULSE_HIGH {
                //     return presses;
                // }
                let Some(module) = self.get_mut(to) else {
                    continue;
                };

                let Some(next_pulse) = module.next(from, pulse) else {
                    continue;
                };

                if to == v.0 && next_pulse == PULSE_HIGH {
                    presses.0 = presses.0.min(c);
                }

                if to == v.1 && next_pulse == PULSE_HIGH {
                    presses.1 = presses.1.min(c);
                }

                if to == v.2 && next_pulse == PULSE_HIGH {
                    presses.2 = presses.2.min(c);
                }
                
                if to == v.3 && next_pulse == PULSE_HIGH {
                    presses.3 = presses.3.min(c);
                }

                if presses.0 < usize::MAX && presses.1 < usize::MAX && presses.2 < usize::MAX && presses.3 < usize::MAX {
                    return presses
                }

                for dest in self.get_destinations(to) {
                    queue.push_back((to, dest, next_pulse))
                }
            }
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            writeln!(f, "{:>64b}", self.mods[i].state)?
        }
        Ok(())
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[test]
fn lcm_works() {
    assert_eq!(lcm(5,9), 45);
    assert_eq!(lcm(4,8), 8);
    assert_eq!(lcm(3,4), 12);
    assert_eq!(lcm(10,12), 60);
}
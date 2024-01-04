use std::collections::{HashMap, VecDeque};

pub fn run(input: &str) -> usize {
    let mut modules = parse_input(input);

    // contains (sender, reciever, signal)
    let mut messages = VecDeque::new();
    let mut cycles: HashMap<&str, usize> = match modules.get("jm") {
        Some(ModuleMap {
            module: Module::Conjunction(map),
            ..
        }) => map.keys().map(|&k| (k, 0)).collect(),
        _ => panic!("jm not found"),
    };

    let mut i = 0;
    loop {
        messages.push_back(("", "broadcaster", false));
        while !messages.is_empty() {
            let (sender, receiver, signal) = messages.pop_front().unwrap();

            // if id not in modules, continue
            if !modules.contains_key(receiver) {
                continue;
            }

            let module_map = modules.get_mut(receiver).unwrap();

            // push outputs to queue
            if send_signals(
                module_map,
                sender,
                receiver,
                signal,
                &mut messages,
                &mut cycles,
                i,
            ) {
                return cycles.values().fold(1, |acc, &v| lcm(acc, v));
            }
        }

        i += 1;
    }
}

// returns true if all cycles have been found
fn send_signals<'a>(
    module_map: &mut ModuleMap<'a>,
    sender: &'a str,
    receiver: &'a str,
    signal: bool,
    messages: &mut VecDeque<(&'a str, &'a str, bool)>,
    cycles: &mut HashMap<&'a str, usize>,
    counter: usize,
) -> bool {
    let ModuleMap { module, outputs } = module_map;
    match module {
        Module::Broadcaster => {
            for output in outputs {
                messages.push_back((receiver, output, signal));
            }
        }
        Module::FlipFlop(ref mut state) => {
            if !signal {
                // flip the state
                *state = !*state;
                for output in outputs {
                    messages.push_back((receiver, output, *state));
                }
            }
        }
        Module::Conjunction(map) => {
            // update map
            map.insert(sender, signal);
            let all_high = map.values().all(|&v| v);

            if receiver == "jm" {
                for (&k, &v) in map.iter() {
                    if cycles[k] == 0 && v {
                        cycles.insert(k, counter + 1);
                    }
                }

                if cycles.values().all(|&v| v != 0) {
                    return true;
                }
            }

            for output in outputs {
                messages.push_back((receiver, output, !all_high));
            }
        }
    }
    false
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster,
    FlipFlop(bool),                      // on/off
    Conjunction(HashMap<&'a str, bool>), // on/off
}

#[derive(Debug, Clone)]
struct ModuleMap<'a> {
    module: Module<'a>,
    outputs: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, ModuleMap> {
    let mut modules = HashMap::new();
    for line in input.lines() {
        let (id, module) = line.split_once(" -> ").unwrap();
        let outputs = module.split(", ").collect();
        match id.as_bytes()[0] {
            b'%' => {
                let module = Module::FlipFlop(false);
                modules.insert(&id[1..], ModuleMap { module, outputs });
            }
            b'&' => {
                let module = Module::Conjunction(HashMap::new());
                modules.insert(&id[1..], ModuleMap { module, outputs });
            }
            // otherwise, it's the broadcaster
            _ => {
                let module = Module::Broadcaster;
                modules.insert(id, ModuleMap { module, outputs });
            }
        }
    }

    // I have to copy :(
    for (id, module) in modules.clone().iter_mut() {
        for output in &module.outputs {
            if let Some(ModuleMap {
                module: Module::Conjunction(map),
                ..
            }) = &mut modules.get_mut(output)
            {
                map.insert(id, false);
            }
        }
    }
    modules
}

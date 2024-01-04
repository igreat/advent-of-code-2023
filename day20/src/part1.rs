use std::collections::{HashMap, VecDeque};

// todo: be wary of cycles

pub fn run(input: &str) -> usize {
    let mut modules = parse_input(input);

    let (mut low_pulses, mut high_pulses) = (0, 0);
    let mut current_ids = VecDeque::new();

    let num_iterations = 1000;
    for _ in 0..num_iterations {
        low_pulses += 1; // pressing the button

        // let contains (sender, reciever, signal)
        current_ids.push_back(("", "broadcaster", false));
        while !current_ids.is_empty() {
            let (sender, receiver, signal) = current_ids.pop_front().unwrap();

            // if id not in modules, continue
            if !modules.contains_key(receiver) {
                continue;
            }

            // push outputs to queue
            let ModuleMap { module, outputs } = modules.get_mut(receiver).unwrap();
            match module {
                Module::Broadcaster => {
                    for output in outputs {
                        if signal {
                            high_pulses += 1;
                        } else {
                            low_pulses += 1;
                        }

                        current_ids.push_back((receiver, output, signal));
                    }
                }
                Module::FlipFlop(ref mut state) => {
                    if !signal {
                        // flip the state
                        *state = !*state;
                        for output in outputs {
                            if *state {
                                high_pulses += 1;
                            } else {
                                low_pulses += 1;
                            }

                            current_ids.push_back((receiver, output, *state));
                        }
                    }
                }
                Module::Conjunction(map) => {
                    // update map
                    map.insert(sender, signal);

                    // check if all signals are high
                    let all_high = map.values().all(|&v| v);
                    for output in outputs {
                        if !all_high {
                            high_pulses += 1;
                        } else {
                            low_pulses += 1;
                        }

                        current_ids.push_back((receiver, output, !all_high));
                    }
                }
            }
        }
    }

    low_pulses * high_pulses
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
        if let Module::FlipFlop(_) = module.module {
            for output in &module.outputs {
                if let Module::Conjunction(map) = &mut modules.get_mut(output).unwrap().module {
                    map.insert(id, false);
                }
            }
        }
    }
    modules
}

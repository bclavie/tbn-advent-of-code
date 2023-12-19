use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let inputs = include_str!("inputs/19");
    let (workflows_in, ratings_in) = inputs.split_once("\n\n").unwrap();

    let mut workflows = HashMap::<String, Workflow>::new();
    for line in workflows_in.lines() {
        let (name, ops) = line[0..line.len() - 1].split_once('{').unwrap();

        let workflow_items: Vec<WorkflowItem> = ops
            .split(',')
            .map(|s| {
                if s.contains(':') {
                    WorkflowItem::Comparison(Comparison::from_str(s))
                } else {
                    WorkflowItem::Outcome(Outcome::from_str(s))
                }
            })
            .collect();

        let workflow = Workflow { workflow_items };
        workflows.insert(name.to_string(), workflow);
    }

    let ratings: Vec<Rating> = ratings_in.lines().map(Rating::from_str).collect();
    let mut accepted: Vec<Rating> = vec![];

    for r in ratings {
        let first = workflows.get("in").unwrap();
        let mut outcome: Outcome = first.do_workflow(r);
        loop {
            match outcome {
                Outcome::Accepted => {
                    accepted.push(r);
                    break;
                }
                Outcome::Rejected => break,
                Outcome::WorkflowName(wf_name) => {
                    let wf = workflows.get(&wf_name).unwrap();
                    outcome = wf.do_workflow(r);
                }
            }
        }
    }

    let part1: i64 = accepted.iter().map(|r| r.score()).sum();
    println!("part1 {}", part1);

    // Brute forcing didn't work, so actually need to do a proper thing.
    // Work with restrictions, rather than concrete numbers.
    // traverse the workflow "tree" down to find restrictions that can result in acceptable nodes.
    let mut acceptable_restrictions: Vec<Restriction> = vec![];

    let starting_restriction = Restriction {
        x_bounds: Interval { min: 1, max: 4000 },
        m_bounds: Interval { min: 1, max: 4000 },
        a_bounds: Interval { min: 1, max: 4000 },
        s_bounds: Interval { min: 1, max: 4000 },
    };
    find_restrictions(
        "in".to_string(),
        starting_restriction,
        &workflows,
        &mut acceptable_restrictions,
    );

    let part2: u64 = acceptable_restrictions.iter().map(|r| r.score()).sum();

    println!("part2 {}", part2);
}

fn find_restrictions(
    current_name: String,
    current_restriction: Restriction,
    wfs: &HashMap<String, Workflow>,
    acceptable_restrictions: &mut Vec<Restriction>,
) {
    let wf = wfs.get(&current_name).unwrap();
    // workflows go from left to right, so more and more restrictions
    // are added as go from left to right;
    let mut culmulative_restrictions = current_restriction;

    for (i, item) in wf.workflow_items.iter().enumerate() {
        match item {
            // an Outcome is awlays the last one..
            WorkflowItem::Outcome(outcome) => {
                let len = wf.workflow_items.len();
                debug_assert_eq!(i, len - 1);
                match outcome {
                    Outcome::Accepted => {
                        acceptable_restrictions.push(culmulative_restrictions);
                        return;
                    }
                    Outcome::Rejected => return,
                    Outcome::WorkflowName(name) => find_restrictions(
                        name.clone(),
                        culmulative_restrictions,
                        wfs,
                        acceptable_restrictions,
                    ),
                }
            }
            WorkflowItem::Comparison(comparison) => {
                let mut new_restriction = culmulative_restrictions;
                // Set the new restriction.
                let new_interval = match comparison.op {
                    Operation::GT => Interval {
                        min: comparison.value + 1,
                        max: 4000,
                    },
                    Operation::LT => Interval {
                        min: 1,
                        max: comparison.value - 1,
                    },
                };

                let inverted_interval = match comparison.op {
                    Operation::GT => Interval {
                        min: 1,
                        max: comparison.value,
                    },
                    Operation::LT => Interval {
                        min: comparison.value,
                        max: 4000,
                    },
                };

                let is_possible =
                    new_restriction.with_new_interval(comparison.system, new_interval);
                // Then go do the outcome if possible.
                if is_possible {
                    match &comparison.outcome {
                        Outcome::Accepted => {
                            acceptable_restrictions.push(new_restriction);
                        }
                        Outcome::Rejected => {}
                        Outcome::WorkflowName(new_name) => find_restrictions(
                            new_name.clone(),
                            new_restriction,
                            wfs,
                            acceptable_restrictions,
                        ),
                    }
                }

                // Handle updating the updated "inverted" restriction.
                // I don't think this can fail...
                debug_assert!(culmulative_restrictions
                    .with_new_interval(comparison.system, inverted_interval));
            }
        }
    }
}

// Restriction is a condition on xmas systems in order to land on an acceptable endpoint.
// I don't think we can end up with disjoint intervals for any given acceptable node.
#[derive(Debug, Copy, Clone)]
struct Restriction {
    x_bounds: Interval,
    m_bounds: Interval,
    a_bounds: Interval,
    s_bounds: Interval,
}

impl Restriction {
    fn score(&self) -> u64 {
        self.x_bounds.size() * self.m_bounds.size() * self.a_bounds.size() * self.s_bounds.size()
    }

    fn with_new_interval(&mut self, system: System, interval: Interval) -> bool {
        match system {
            System::X => {
                if do_intervals_overlap(interval, self.x_bounds) {
                    self.x_bounds = interval_overlaps(interval, self.x_bounds);
                    true
                } else {
                    // Impossible outcome, so no need to explore this branch.
                    false
                }
            }
            System::M => {
                if do_intervals_overlap(interval, self.m_bounds) {
                    self.m_bounds = interval_overlaps(interval, self.m_bounds);
                    true
                } else {
                    // Impossible outcome, so no need to explore this branch.
                    false
                }
            }
            System::A => {
                if do_intervals_overlap(interval, self.a_bounds) {
                    self.a_bounds = interval_overlaps(interval, self.a_bounds);
                    true
                } else {
                    // Impossible outcome, so no need to explore this branch.
                    false
                }
            }
            System::S => {
                if do_intervals_overlap(interval, self.s_bounds) {
                    self.s_bounds = interval_overlaps(interval, self.s_bounds);
                    true
                } else {
                    // Impossible outcome, so no need to explore this branch.
                    false
                }
            }
        }
    }
}

// These are inclusive intervals
#[derive(Debug, Copy, Clone)]
struct Interval {
    min: i64,
    max: i64,
}

impl Interval {
    fn size(&self) -> u64 {
        (self.max - self.min + 1) as u64
    }
}

// Returns a new interval which is the overlap of the two input intervals.
fn interval_overlaps(a: Interval, b: Interval) -> Interval {
    if !do_intervals_overlap(a, b) {
        panic!("intervals do not overlap")
    }
    Interval {
        min: a.min.max(b.min),
        max: a.max.min(b.max),
    }
}

fn do_intervals_overlap(a: Interval, b: Interval) -> bool {
    (a.min..=a.max).contains(&b.max) || (a.min..=a.max).contains(&b.min)
}

#[derive(Debug, Copy, Clone)]
struct Rating {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Rating {
    fn from_str(s: &str) -> Self {
        let t = &s[1..s.len() - 1];
        let (x_str, m_str, a_str, s_str) = t.splitn(4, ',').collect_tuple().unwrap();

        let x = x_str.split_once('=').unwrap().1.parse::<i64>().unwrap();
        let m = m_str.split_once('=').unwrap().1.parse::<i64>().unwrap();
        let a = a_str.split_once('=').unwrap().1.parse::<i64>().unwrap();
        let s = s_str.split_once('=').unwrap().1.parse::<i64>().unwrap();

        Self { x, m, a, s }
    }

    fn score(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    workflow_items: Vec<WorkflowItem>,
}

impl Workflow {
    fn do_workflow(&self, rating: Rating) -> Outcome {
        for item in self.workflow_items.iter() {
            match item {
                WorkflowItem::Comparison(comp) => match comp.op {
                    Operation::GT => match comp.system {
                        System::X => {
                            if rating.x > comp.value {
                                return comp.outcome.clone();
                            }
                        }
                        System::M => {
                            if rating.m > comp.value {
                                return comp.outcome.clone();
                            }
                        }
                        System::A => {
                            if rating.a > comp.value {
                                return comp.outcome.clone();
                            }
                        }
                        System::S => {
                            if rating.s > comp.value {
                                return comp.outcome.clone();
                            }
                        }
                    },
                    Operation::LT => match comp.system {
                        System::X => {
                            if rating.x < comp.value {
                                return comp.outcome.clone();
                            }
                        }
                        System::M => {
                            if rating.m < comp.value {
                                return comp.outcome.clone();
                            }
                        }
                        System::A => {
                            if rating.a < comp.value {
                                return comp.outcome.clone();
                            }
                        }
                        System::S => {
                            if rating.s < comp.value {
                                return comp.outcome.clone();
                            }
                        }
                    },
                },
                WorkflowItem::Outcome(outcome) => return outcome.clone(),
            }
        }

        panic!("unexpected result")
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum WorkflowItem {
    Outcome(Outcome),
    Comparison(Comparison),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Outcome {
    Accepted,
    Rejected,
    WorkflowName(String),
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Outcome::Accepted,
            "R" => Outcome::Rejected,
            _ => Outcome::WorkflowName(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Comparison {
    system: System,
    op: Operation,
    value: i64,
    outcome: Outcome,
}
impl Comparison {
    fn from_str(s: &str) -> Self {
        let (comp, res) = s.split_once(':').unwrap();

        let system = System::from_char(comp.chars().nth(0).unwrap());
        let op = Operation::from_char(comp.chars().nth(1).unwrap());
        let value = comp[2..comp.len()].parse::<i64>().unwrap();

        let outcome = Outcome::from_str(res);

        Self {
            system,
            op,
            value,
            outcome,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum System {
    X,
    M,
    A,
    S,
}
impl System {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("unexpected char {}", c),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    GT,
    LT,
}

impl Operation {
    fn from_char(c: char) -> Self {
        match c {
            '>' => Self::GT,
            '<' => Self::LT,
            _ => panic!("unexpected char {}", c),
        }
    }
}

use rustc_hash::FxHashSet;

pub fn solve_fn<F, S, II, I>(mut next: F, mut states: Vec<S>)
where
    F: FnMut(&S) -> II,
    II: IntoIterator<Item = S, IntoIter = I>,
    I: Iterator<Item = S> + DoubleEndedIterator,
{
    while let Some(mut current) = states.pop() {
        // An inner loop for the first entry which avoids a push with an immediate pop.
        loop {
            let mut iter = next(&current).into_iter();
            match iter.next() {
                Some(n) => {
                    current = n;
                }
                None => break,
            }
            states.extend(iter.rev());
        }
    }
}

/// Push/pop stack implementation with an optimization for the last entry.
pub struct StateStack<S> {
    next: Option<S>,
    states: Vec<S>,
}
impl<S> StateStack<S> {
    pub fn push(&mut self, state: S) {
        if self.next.is_none() {
            self.next = Some(state);
        } else {
            self.states.push(state);
        }
    }
    fn pop(&mut self) -> Option<S> {
        self.next.take().or_else(|| self.states.pop())
    }
}

pub fn solve_fn_push<F, S>(mut next: F, states: Vec<S>)
where
    F: FnMut(&mut StateStack<S>, &S),
{
    let mut stack = StateStack { next: None, states };

    while let Some(current) = stack.pop() {
        next(&mut stack, &current);
    }
}

pub enum NodeResult {
    Stop,
    Next,
}

pub fn solve_breadth<F, S>(mut next: F, mut states: Vec<S>) -> (S, usize)
where
    F: FnMut(&S, usize, &mut FxHashSet<S>) -> NodeResult,
    S: std::hash::Hash + Eq,
{
    let mut round = 0;
    loop {
        let mut next_states = FxHashSet::with_capacity_and_hasher(states.len(), Default::default());
        for state in states {
            match next(&state, round, &mut next_states) {
                NodeResult::Stop => return (state, round),
                NodeResult::Next => {}
            };
        }
        if next_states.is_empty() {
            panic!("Result not found")
        }
        states = next_states.into_iter().collect();
        round += 1;
    }
}

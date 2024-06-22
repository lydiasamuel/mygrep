/* Intuition from Wikipedia (https://en.wikipedia.org/wiki/Powerset_construction):
 * To simulate the operation of a DFA on a given input string, one needs to keep track of a single
 * state at any time: the state that the automaton will reach after seeing a prefix of the input.
 * In contrast, to simulate an NFA, one needs to keep track of a set of states: all of the states
 * that the automaton could reach after seeing the same prefix of the input, according to the nondeterministic
 * choices made by the automaton. If, after a certain prefix of the input, a set S of states can be
 * reached, then after the next input symbol x the set of reachable states is a deterministic function
 * of S and x. Therefore, the sets of reachable NFA states play the same role in the NFA simulation as
 * single DFA states play in the DFA simulation, and in fact the sets of NFA states appearing in this
 * simulation may be re-interpreted as being states of a DFA.
 *
 * My Explanation:
 * We construct a deterministic automata from a non-deterministic automata by simulating the non-determinism
 * as determinism. Since the computer doesn't have guessware we can't know which path to take, so to simulate
 * the non-determinism we have to just try all the possibilities available at each point.
 *
 * The procedure is as follows, we group all equivalently reachable states of the NFA after seeing a prefix
 * of the input as a single node of the final DFA. Then we link that node up to the previous one by the
 * actual transition it took to get to it, discarding all the possible empty transitions we had to try.
 *
 * Psuedocode:
 * q0 = eClosure({n0});
 * Q = q0
 * worklist = {q0}
 * while (worklist is not empty)
 *  remove q from worklist
 *  for each char in alphabet
 *    t = eClosure(Delta(q, c))
 *    if t is not empty
 *      T[q,c] = t
 *      if t is not in Q then
 *        add t to Q and worklist
 *
 * Note: delta gets the neighbours of each sub state by taking an edge labelled with char c from it.
 */

use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::{
    automata::{AutomataComponent, AutomataLabel, AutomataState},
    graph::{Graph, NodeIndex},
};

// Use a BTreeSet because it implements Hash since it stores it's elements in sorted order.
type DFAState = BTreeSet<NodeIndex>;

pub fn build_dfa(
    handle: AutomataComponent,
    nfa: Graph<AutomataState, AutomataLabel>,
    alphabet: Vec<char>,
) -> (NodeIndex, Graph<AutomataState, char>) {
    let mut dfa: Graph<AutomataState, char> = Graph::new();

    let start = handle.get_start_state();
    let accept = handle.get_accept_state();

    let start_of_dfa = Rc::new(empty_closure(&nfa, Rc::new(BTreeSet::from([start]))));
    let is_start_accepting = start_of_dfa.contains(&accept);
    let start_index = dfa.add_node(AutomataState::new(is_start_accepting));

    let mut final_dfa_edges: HashSet<(NodeIndex, NodeIndex, char)> = HashSet::new();
    let mut final_dfa_states: HashMap<Rc<DFAState>, NodeIndex> =
        HashMap::from([(start_of_dfa.clone(), start_index)]);

    let mut worklist: VecDeque<(Rc<DFAState>, NodeIndex)> = VecDeque::new();
    worklist.push_back((start_of_dfa.clone(), start_index));

    while !worklist.is_empty() {
        let (current, index) = worklist.pop_front().unwrap();

        for c in alphabet.iter() {
            let available_neighbours = Rc::new(delta(&nfa, current.clone(), *c));
            let next = Rc::new(empty_closure(&nfa, available_neighbours.clone()));

            // Guard against adding empty states, i.e. the delta and empty closure returned nothing so there's no deterministic transition to be made on c
            if !next.is_empty() {
                let next_index: usize;

                if !final_dfa_states.contains_key(&next) {
                    let is_next_accepting = next.contains(&accept);
                    next_index = dfa.add_node(AutomataState::new(is_next_accepting));

                    final_dfa_states.insert(next.clone(), next_index);
                    worklist.push_back((next.clone(), next_index));
                } else {
                    next_index = *final_dfa_states.get(&next).unwrap();
                }

                // Guard against adding duplicate edges
                if !final_dfa_edges.contains(&(index, next_index, *c)) {
                    final_dfa_edges.insert((index, next_index, *c));
                    dfa.add_edge(index, next_index, *c);
                }
            }
        }
    }

    return (start_index, dfa);
}

// Using a depth-first search here to do the empty closure
fn empty_closure(nfa: &Graph<AutomataState, AutomataLabel>, from: Rc<DFAState>) -> DFAState {
    let mut result: DFAState = BTreeSet::new();
    let mut visit_stack: Vec<NodeIndex> = Vec::new();

    for state in from.iter() {
        visit_stack.push(*state);
    }

    while !visit_stack.is_empty() {
        let current = visit_stack.pop().unwrap();
        result.insert(current);

        let outgoing_edges = nfa.outgoing_edges(current).unwrap();

        for edge in outgoing_edges {
            let data = nfa.get_edge_data(&edge).unwrap().clone();
            let label = (*data).borrow().get_label();

            if label == None {
                let next = nfa.traverse(edge).unwrap();

                if !result.contains(&next) && !visit_stack.contains(&next) {
                    visit_stack.push(next)
                }
            }
        }
    }

    return result;
}

fn delta(nfa: &Graph<AutomataState, AutomataLabel>, from: Rc<DFAState>, c: char) -> DFAState {
    let mut result: BTreeSet<NodeIndex> = BTreeSet::new();

    for state in (*from).iter() {
        let outgoing_edges = nfa.outgoing_edges(*state).unwrap();

        for edge in outgoing_edges {
            let data = nfa.get_edge_data(&edge).unwrap().clone();
            let label = (*data).borrow().get_label();

            match label {
                Some(s) if s == c => {
                    let target = nfa.traverse(edge).unwrap();
                    result.insert(target);
                }
                _ => (),
            }
        }
    }

    return result;
}

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
 *    T[q,c] = t
 *    if t is not in Q then
 *      add t to Q and worklist
 *
 * Note: delta gets the neighbours of each sub state by taking an edge labelled with char c from it.
 */

use std::collections::{BTreeSet, VecDeque, HashMap};

use crate::{graph::{NodeIndex, Graph}, automata::{AutomataState, AutomataLabel}};

// Use a BTreeSet because it implements Hash since it stores it's elements in sorted order.
type DFAState = BTreeSet<NodeIndex>;

pub fn build_dfa(start: NodeIndex, nfa: Graph<AutomataState, AutomataLabel>, alphabet: Vec<char>) -> (NodeIndex, Graph<AutomataState, char>) {
    let mut dfa: Graph<AutomataState, char> = Graph::new();
    
    let start_of_dfa = empty_closure(&nfa, &BTreeSet::from([start]));

    let is_start_accepting = contains_accepting_state(&start_of_dfa, &nfa);
    let start_index = dfa.add_node(AutomataState::new(is_start_accepting));

    let mut final_dfa_states: HashMap<DFAState, NodeIndex> = HashMap::from([(start_of_dfa, start_index)]);
    
    let mut worklist: VecDeque<(DFAState, NodeIndex)> = VecDeque::new();
    worklist.push_back((start_of_dfa, start_index));

    while !worklist.is_empty() {
        let (current, index) = worklist.pop_front().unwrap();

        for c in alphabet {
            let available_neighbours = delta(&nfa,  &current, c);
            let next = empty_closure(&nfa, &available_neighbours);

            let next_index: usize;

            if !final_dfa_states.contains_key(&next) {
                let is_next_accepting = contains_accepting_state(&start_of_dfa, &nfa);
                next_index = dfa.add_node(AutomataState::new(is_next_accepting));

                final_dfa_states.insert(next, next_index);
                worklist.push_back((next, next_index));
            }
            else {
                next_index = *final_dfa_states.get(&next).unwrap();
            }

            dfa.add_edge(index, next_index, c);
        }
    }

    return (start_index, dfa)
}

fn empty_closure(nfa: &Graph<AutomataState, AutomataLabel>, states: &DFAState) -> DFAState {

}

fn delta(nfa: &Graph<AutomataState, AutomataLabel>, states: &DFAState, c: char) -> DFAState {
    let mut result: BTreeSet<NodeIndex> = BTreeSet::new();

    for sub_state in states {
        let outgoing_edges = nfa.outgoing_edges(*sub_state).unwrap();

        for edge in outgoing_edges {
            let data = nfa.get_edge_data(edge).unwrap().borrow();
            
            if let Some(s) = data.get_label() {
                if s == c {
                    result.insert(nfa.traverse(edge).unwrap());
                }
            }
        }
    }

    return result;
}

fn contains_accepting_state(states: &DFAState, nfa: &Graph<AutomataState, AutomataLabel>) -> bool {
    for sub_state in states {
        let data = nfa.get_node_data(*sub_state).unwrap();

        if data.borrow().is_accepting() {
            return true;
        }
    }

    return false;
}
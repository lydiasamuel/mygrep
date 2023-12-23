/* https://en.wikipedia.org/wiki/Powerset_construction
 *
 * Intuition from Wikipedia:
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
 * My Words:
 * Basically the algorithm starts at the beginning of the NFA and builds the DFA by simulating the
 * equivalent deterministic edges one at a time over all the alphabet until there's nothing left to
 * simulate. 
 * 
 * i.e. You can get to x, y and z by consuming these empty transitions and then taking an edge
 * labelled e.g. 'i' therefore deterministically they are the same.
 * 
 * The states in which the NFA could be in after taking an edge labelled e.g. 'i' (accounting for all 
 * the empty transitions it could have taken) are grouped as one and become a state in the final DFA.
 * Note that we account for all the empty transitions because we don't know which empty transition 
 * the NFA might take, so to determine things we've got to try all of them.
 * 
 * The algorithm takes these new super states of the DFA and repeats the process of simulating 
 * deterministic edges from each of the sub states over the alphabet. This gets the next lot
 * of super states from the current one, with the corresponding labelled edges to them.
 * 
 * Rinse and repeat until you've simulated everything, and if a final state of the NFA is in a super state,
 * that means that the super state could accept.
 * 
 * One sentence:
 * Group all equivalently reachable states of the NFA, after seeing a prefix of the input as a single 
 * node of the final DFA, and link that node up by the deterministic transition it took to get to it.
 */

/* Psuedo code:
 * start_of_dfa = empty_closure({start_of_nfa})
 * final_dfa_states = {start_of_dfa}
 * final_dfa_edges = {}
 * worklist = {start_of_dfa}
 * while (worklist is not empty)
 *   remove current_state from worklist
 *   for each char c in alphabet
 *     next_state = empty_closure(delta(current_state, c))
 *     add to final_dfa_edges: (current_state -> next_state) on char c
 *     if any sub state is accepting in next_state
 *       mark next_state as accepting 
 *     if next_state is not in final_dfa_states then
 *       add next_state to final_dfa_states 
 *       add next_state to worklist
 * 
 * Note: delta gets the neighbours of each sub state by taking an edge labelled with char c from it.
 */

use std::collections::HashSet;

use crate::{graph::{NodeIndex, Graph}, automata::{AutomataState, AutomataLabel}};

type DFAState = HashSet<NodeIndex>;

pub fn build_dfa(start: NodeIndex, nfa: Graph<AutomataState, AutomataLabel>, alphabet: Vec<char>) -> (NodeIndex, Graph<AutomataState, AutomataLabel>) {
    let start_of_dfa = empty_closure(&nfa, &vec![start]);

    let final_dfa_states: HashSet<DFAState> = HashSet::from([start_of_dfa]);
}

fn empty_closure(nfa: &Graph<AutomataState, AutomataLabel>, states: &DFAState) -> DFAState {

}

fn delta(nfa: &Graph<AutomataState, AutomataLabel>, states: &DFAState, c: char) -> DFAState {

}
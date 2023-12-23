use std::collections::VecDeque;

use crate::graph::{Graph, NodeIndex};
use crate::automata::{AutomataState, AutomataComponent, AutomataLabel};
use crate::regex::RegexSymbol;

// Using Thompson construction of the NFA from postfix regex
// The final NFA will have exactly one initial state and one final accepting state
// Link: https://en.wikipedia.org/wiki/Thompson%27s_construction
pub fn build_nfa(postfix_regex: VecDeque<RegexSymbol>) -> (NodeIndex, Graph<AutomataState, AutomataLabel>) {
    let mut nfa: Graph<AutomataState, AutomataLabel> = Graph::new();
    let mut component_stack: Vec<AutomataComponent> = Vec::new();

    for symbol in postfix_regex {
        let component = compile(&mut nfa, &mut component_stack, symbol);
        component_stack.push(component);
    }

    let result = component_stack.pop().unwrap();

    // Mark final state as accepting
    nfa.get_node_data(result.get_accept_state())
        .unwrap()
        .borrow_mut()
        .mark_as_accepting();

    return (result.get_start_state(), nfa);
}

fn compile(nfa: &mut Graph<AutomataState, AutomataLabel>, component_stack: &mut Vec<AutomataComponent>, symbol: RegexSymbol) -> AutomataComponent {
    match symbol {
        RegexSymbol::Optional => return compile_optional(nfa, component_stack),
        RegexSymbol::Plus => return compile_plus(nfa, component_stack),
        RegexSymbol::Star => return compile_star(nfa, component_stack),
        RegexSymbol::Concat => return compile_concat(nfa, component_stack),
        RegexSymbol::Alternation => return compile_alternation(nfa, component_stack),
        RegexSymbol::Char(c) => return compile_character(nfa, c),
        _ => panic!("Error - Parenthesis should have been removed in postfixing stage!")
    }
}

fn compile_character(nfa: &mut Graph<AutomataState, AutomataLabel>, c: char) -> AutomataComponent {
    let start = nfa.add_node(AutomataState::new(false));
    let accept = nfa.add_node(AutomataState::new(false));

    nfa.add_edge(start, accept, AutomataLabel::new(Some(c)));

    return AutomataComponent::new(start, accept);
}

fn compile_optional(nfa: &mut Graph<AutomataState, AutomataLabel>, component_stack: &mut Vec<AutomataComponent>) -> AutomataComponent {
    let top = component_stack.pop().unwrap();
    
    let start = nfa.add_node(AutomataState::new(false));
    let accept = nfa.add_node(AutomataState::new(false));

    nfa.add_edge(start, top.get_start_state(), AutomataLabel::new(None));
    nfa.add_edge(top.get_accept_state(), accept, AutomataLabel::new(None));
    nfa.add_edge(start, accept, AutomataLabel::new(None));

    return AutomataComponent::new(start, accept);
}

fn compile_plus(nfa: &mut Graph<AutomataState, AutomataLabel>, component_stack: &mut Vec<AutomataComponent>) -> AutomataComponent {
    let top = component_stack.pop().unwrap();
    
    let start = nfa.add_node(AutomataState::new(false));
    let accept = nfa.add_node(AutomataState::new(false));

    nfa.add_edge(start, top.get_start_state(), AutomataLabel::new(None));
    nfa.add_edge(top.get_accept_state(), accept, AutomataLabel::new(None));
    nfa.add_edge(top.get_accept_state(), top.get_start_state(), AutomataLabel::new(None));

    return AutomataComponent::new(start, accept);
}

fn compile_star(nfa: &mut Graph<AutomataState, AutomataLabel>, component_stack: &mut Vec<AutomataComponent>) -> AutomataComponent {
    let top = component_stack.pop().unwrap();
    
    let start = nfa.add_node(AutomataState::new(false));
    let accept = nfa.add_node(AutomataState::new(false));

    nfa.add_edge(start, top.get_start_state(), AutomataLabel::new(None));
    nfa.add_edge(top.get_accept_state(), accept, AutomataLabel::new(None));
    nfa.add_edge(top.get_accept_state(), top.get_start_state(), AutomataLabel::new(None));
    nfa.add_edge(start, accept, AutomataLabel::new(None));

    return AutomataComponent::new(start, accept);
}

fn compile_concat(nfa: &mut Graph<AutomataState, AutomataLabel>, component_stack: &mut Vec<AutomataComponent>) -> AutomataComponent {
    let right = component_stack.pop().unwrap();
    let left = component_stack.pop().unwrap();

    nfa.add_edge(left.get_accept_state(), right.get_start_state(), AutomataLabel::new(None));

    return AutomataComponent::new(left.get_start_state(), right.get_accept_state());
}

fn compile_alternation(nfa: &mut Graph<AutomataState, AutomataLabel>, component_stack: &mut Vec<AutomataComponent>) -> AutomataComponent {
    let right = component_stack.pop().unwrap();
    let left = component_stack.pop().unwrap();

    let start = nfa.add_node(AutomataState::new(false));
    let accept = nfa.add_node(AutomataState::new(false));

    nfa.add_edge(start, left.get_start_state(), AutomataLabel::new(None));
    nfa.add_edge(start, right.get_start_state(), AutomataLabel::new(None));
    nfa.add_edge(left.get_accept_state(), accept, AutomataLabel::new(None));
    nfa.add_edge(right.get_accept_state(), accept, AutomataLabel::new(None));

    return AutomataComponent::new(start, accept);
}
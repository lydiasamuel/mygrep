use mygrep::{postfixer, nfa::build_nfa, dfa::build_dfa, regex::get_alphabet};

fn main() {
    let postfix_regex = postfixer::transform("(a|b)a*".to_string()).unwrap();

    let alphabet = get_alphabet(&postfix_regex);

    let (handle, nfa) = build_nfa(postfix_regex);

    let (start, dfa) = build_dfa(handle, nfa, alphabet);
    
    for source in 0..dfa.num_of_nodes() {
        let outgoing_edges = dfa.outgoing_edges(source).unwrap();

        print!("{}: ", source);

        for edge in outgoing_edges {
            let data = dfa.get_edge_data(&edge).unwrap();
            let label: char = *data.borrow();
            let target = dfa.traverse(edge).unwrap();

            print!("{}-'{}'->{} ", source, label, target);
        }
        
        println!();
    }

    /*
    for source in 0..nfa.num_of_nodes() {
        let outgoing_edges = nfa.outgoing_edges(source).unwrap();
        
        if source == handle.get_start_state() {
            print!("Start ")
        }

        if source == handle.get_accept_state() {
            print!("Accepting ")
        }

        print!("{}: ", source);

        for edge in outgoing_edges {
            let data = nfa.get_edge_data(&edge).unwrap();
            let label: Option<char> = data.borrow().get_label();
            let target = nfa.traverse(edge).unwrap();

            if let Some(c) = label {
                print!("{}-'{}'->{} ", source, c, target);
            }
            else {
                print!("{}->{} ", source, target);
            }
        }
        
        println!();
    }
    */
}
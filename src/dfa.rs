    // q0 = eClosure({n0});
    // Q = q0
    // worklist = {q0}
    // while (worklist is not empty)
    //  remove q from worklist
    //  for each char in alphabet
    //      t = eClosure(Delta(q, c))
    //      T[q,c] = t
    //      if t is not in Q then
    //          add t to Q and worklist
    //
    // Delta computes the new states possible from each element in q

fn get_alphabet(regex: VecDeque<RegexSymbol>) -> HashSet<char> {
    let mut alphabet: HashSet<char> = HashSet::new();

    for symbol in regex {
        if let RegexSymbol::Char(c) = symbol {
            alphabet.insert(c);
        }
    }

    return alphabet;
}

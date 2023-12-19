use crate::graph::NodeIndex;

pub struct AutomataState {
    accepting: bool
}

pub struct AutomataLabel {
    label: Option<char>,
    empty: bool
}

pub struct AutomataComponent {
    start_state: NodeIndex,
    accept_state: NodeIndex
}

impl AutomataState {
    pub fn new(accepting: bool) -> AutomataState {
        return AutomataState {
            accepting
        }
    }

    pub fn is_accepting(&self) -> bool {
        return self.accepting;
    }

    pub fn mark_as_accepting(&mut self) {
        self.accepting = true;
    }
}

impl AutomataLabel {
    pub fn new(label: Option<char>, empty: bool) -> AutomataLabel {
        if empty && label != None {
            panic!("Can't fill in an empty automata label");
        }

        if !empty && label == None {
            panic!("Must fill in a non-empty automata label");
        }

        return AutomataLabel {
            label,
            empty
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.empty;
    }

    pub fn get_label(&self) -> Result<char, &str> {
        if self.empty {
            return Err("Error - No label present on an empty automata label");
        }

        return Ok(self.label.unwrap())
    }
}

impl AutomataComponent {
    pub fn new(start_state: NodeIndex, accept_state: NodeIndex) -> AutomataComponent {
        return AutomataComponent {
            start_state,
            accept_state
        }
    }

    pub fn get_start_state(&self) -> NodeIndex {
        return self.start_state;
    }

    pub fn get_accept_state(&self) -> NodeIndex {
        return self.accept_state;
    }
}
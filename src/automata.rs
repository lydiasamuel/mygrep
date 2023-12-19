use crate::graph::NodeIndex;

pub struct AutomataState {
    accepting: bool
}

pub struct AutomataLabel {
    label: Option<char>
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
    pub fn new(label: Option<char>) -> AutomataLabel {
        return AutomataLabel {
            label
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.label == None;
    }

    pub fn get_label(&self) -> Option<char> {
        return self.label;
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
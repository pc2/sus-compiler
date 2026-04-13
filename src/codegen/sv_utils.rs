use std::{fmt::Display, rc::Rc};

pub struct CommaSeparatedList {
    /// (is_commented, line_text)
    lines: Vec<(bool, String)>,
    comment_text: &'static str,
    indent: &'static str,
}
impl CommaSeparatedList {
    pub fn new(indent: &'static str, comment_text: &'static str) -> Self {
        Self {
            lines: Vec::new(),
            comment_text,
            indent,
        }
    }
    pub fn line(&mut self, line: String) {
        self.lines.push((false, line));
    }
    pub fn commented(&mut self, line: String) {
        self.lines.push((true, line));
    }
}
impl Display for CommaSeparatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = self.indent;
        if self.lines.is_empty() {
            return Ok(());
        }
        writeln!(f)?;
        let last_non_comment_line = self.lines.iter().rposition(|l| !l.0).unwrap_or(0);
        for (idx, (is_commented, line)) in self.lines.iter().enumerate() {
            if *is_commented {
                let c = self.comment_text;
                writeln!(f, "{indent}\t{c}{line}")?;
            } else {
                let comma = if idx < last_non_comment_line { "," } else { "" };
                writeln!(f, "{indent}\t{line}{comma}")?;
            }
        }
        write!(f, "{indent}")
    }
}

pub struct VariableAlloc {
    pub var_names: Vec<Rc<str>>,
    pub currently_used: usize,
    prefix: &'static str,
}
impl VariableAlloc {
    pub fn new(prefix: &'static str) -> Self {
        Self {
            var_names: Vec::new(),
            currently_used: 0,
            prefix,
        }
    }
    pub fn alloc(&mut self) -> Rc<str> {
        let claimed_id = self.currently_used;
        self.currently_used += 1;
        if claimed_id >= self.var_names.len() {
            assert_eq!(claimed_id, self.var_names.len(), "Skipping a var?");
            let prefix = self.prefix;
            self.var_names.push(format!("{prefix}{claimed_id}").into());
        }
        self.var_names[claimed_id].clone()
    }
    /// Does not empty the
    pub fn reuse(&mut self) {
        self.currently_used = 0;
    }
}

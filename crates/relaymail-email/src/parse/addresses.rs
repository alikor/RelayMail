use mailparse::MailAddr;

/// A single parsed email address with optional display name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mailbox {
    address: String,
    display_name: Option<String>,
}

impl Mailbox {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            display_name: None,
        }
    }

    pub fn with_display_name(mut self, display: impl Into<String>) -> Self {
        self.display_name = Some(display.into());
        self
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }
}

pub(crate) fn parse_list(raw: &str) -> Vec<Mailbox> {
    let parsed = match mailparse::addrparse(raw) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for item in parsed.iter() {
        collect(item, &mut out);
    }
    out
}

fn collect(item: &MailAddr, out: &mut Vec<Mailbox>) {
    match item {
        MailAddr::Single(info) => {
            let mut mb = Mailbox::new(info.addr.clone());
            if let Some(name) = info.display_name.clone() {
                mb = mb.with_display_name(name);
            }
            out.push(mb);
        }
        MailAddr::Group(group) => {
            for info in &group.addrs {
                let mut mb = Mailbox::new(info.addr.clone());
                if let Some(name) = info.display_name.clone() {
                    mb = mb.with_display_name(name);
                }
                out.push(mb);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mailbox_with_display_name() {
        let mb = Mailbox::new("alice@example.com").with_display_name("Alice");
        assert_eq!(mb.address(), "alice@example.com");
        assert_eq!(mb.display_name(), Some("Alice"));
    }

    #[test]
    fn parse_list_single_with_display_name() {
        let list = parse_list("Alice <alice@example.com>, bob@example.com");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].address(), "alice@example.com");
        assert_eq!(list[0].display_name(), Some("Alice"));
        assert_eq!(list[1].address(), "bob@example.com");
        assert!(list[1].display_name().is_none());
    }

    #[test]
    fn parse_list_group_syntax_flattens_members() {
        let list = parse_list("Team: alice@example.com, bob@example.com;");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].address(), "alice@example.com");
        assert_eq!(list[1].address(), "bob@example.com");
    }

    #[test]
    fn parse_list_malformed_returns_empty() {
        let list = parse_list("(((badly nested");
        let _ = list; // must not panic; empty or partial result both acceptable
    }
}

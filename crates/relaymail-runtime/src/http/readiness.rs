use std::collections::BTreeMap;
use std::sync::Mutex;

/// Component-level readiness tracker. Endpoints report ready only when every
/// registered component is marked ready.
#[derive(Debug, Default)]
pub struct ReadinessTracker {
    components: Mutex<BTreeMap<String, bool>>,
}

impl ReadinessTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, name: &str) {
        self.components
            .lock()
            .expect("poisoned")
            .insert(name.to_string(), false);
    }

    pub fn mark_ready(&self, name: &str) {
        if let Some(v) = self.components.lock().expect("poisoned").get_mut(name) {
            *v = true;
        }
    }

    pub fn is_ready(&self) -> bool {
        let guard = self.components.lock().expect("poisoned");
        !guard.is_empty() && guard.values().all(|v| *v)
    }

    pub fn snapshot(&self) -> Vec<(String, bool)> {
        let guard = self.components.lock().expect("poisoned");
        guard.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_ready_until_all_marked() {
        let t = ReadinessTracker::new();
        t.register("config");
        t.register("pipeline");
        assert!(!t.is_ready());
        t.mark_ready("config");
        assert!(!t.is_ready());
        t.mark_ready("pipeline");
        assert!(t.is_ready());
    }
}

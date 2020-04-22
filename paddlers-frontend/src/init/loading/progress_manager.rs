use std::any::TypeId;
use std::collections::HashMap;

/// Helper object to manage resources that must be loaded
pub struct ProgressManager {
    total_items: usize,
    loaded: usize,
    loadables: HashMap<TypeId, Loadable>,
}

struct Loadable {
    loaded: usize,
    max: usize,
    msg: &'static str,
}
impl Loadable {
    fn new(max: usize, msg: &'static str) -> Self {
        Loadable {
            loaded: 0,
            max,
            msg,
        }
    }
}

impl ProgressManager {
    pub fn new() -> Self {
        ProgressManager {
            total_items: 0,
            loaded: 0,
            loadables: HashMap::new(),
        }
    }
    /// Add new items that can be loaded, grouped by their type
    pub fn with<T: ?Sized + 'static>(mut self, items: usize, msg: &'static str) -> Self {
        let key = TypeId::of::<T>();
        self.loadables.insert(key, Loadable::new(items, msg));
        self.total_items += items;
        self
    }
    /// Convenienve wrapper for easy type inference, will wait for one item of the type inside the Option
    pub fn with_loadable<T: Sized + 'static>(self, _item: &Option<T>, msg: &'static str) -> Self {
        self.with::<T>(1, msg)
    }
    /// Update the progress tracking with new information. It is okay to report the same progress several times.
    pub fn report_progress<T: ?Sized + 'static>(&mut self, loaded: usize) {
        let key = TypeId::of::<T>();
        let loadable = self.loadables.get_mut(&key).expect("Unknown loadable");
        let before = loadable.loaded;
        if before > loaded {
            println!("Error: progress report less than before");
            return;
        }
        if loaded > loadable.max {
            println!("Error: loaded more than registered");
        }
        let diff = loaded - before;
        self.loaded += diff;
        loadable.loaded = loaded;
    }
    /// Convenience wrapper for easy type inference
    pub fn report_progress_for<T: ?Sized + 'static>(&mut self, _t: &T, loaded: usize) {
        self.report_progress::<T>(loaded)
    }
    pub fn progress(&self) -> f32 {
        self.loaded as f32 / self.total_items as f32
    }
    pub fn done(&self) -> bool {
        self.loaded >= self.total_items
    }
    pub fn waiting_for(&self) -> &'static str {
        for loadable in self.loadables.values() {
            if loadable.loaded < loadable.max {
                return loadable.msg;
            }
        }
        "All done"
    }
}

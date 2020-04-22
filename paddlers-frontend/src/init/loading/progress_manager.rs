use std::any::TypeId;
use std::collections::HashMap;

/// Helper object to manage resources that must be loaded
pub struct ProgressManager {
    total_items: usize,
    loaded: usize,
    progress: HashMap<TypeId, usize>,
}

impl ProgressManager {
    pub fn new() -> Self {
        ProgressManager {
            total_items: 0,
            loaded: 0,
            progress: HashMap::new(),
        }
    }
    /// Add new items that can be loaded, grouped by their type
    pub fn with<T: ?Sized + 'static>(mut self, items: usize) -> Self {
        let key = TypeId::of::<T>();
        self.progress.insert(key, 0);
        self.total_items += items;
        self
    }
    /// Convenienve wrapper for easy type inference, will wait for one item of the type inside the Option
    pub fn with_loadable<T: Sized + 'static>(self, _item: &Option<T>) -> Self {
        self.with::<T>(1)
    }
    /// Update the progress tracking with new information. It is okay to report the same progress several times.
    pub fn report_progress<T: ?Sized + 'static>(&mut self, loaded: usize) {
        let key = TypeId::of::<T>();
        let before = self.progress.insert(key, loaded).expect("Unknown loadable");
        let diff = loaded - before;
        self.loaded += diff;
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
}

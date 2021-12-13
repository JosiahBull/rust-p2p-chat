use serde::{Deserialize, Serialize};

const HISTORY_SIZE: usize = 32;

#[derive(Serialize, Deserialize, Debug)]
pub struct History<T> {
    data: [Option<T>; HISTORY_SIZE], //Store the latest 100 messages
    pointer: usize,
}

#[allow(dead_code)]
impl<T> History<T> {
    ///Create a new history
    pub fn new() -> Self {
        // Workaround due to bug in compiler
        // this would be much cleaner `[None; HISTORY_SIZE]`
        let mut data = vec![];
        for _ in 0..HISTORY_SIZE {
            data.push(None);
        }

        Self {
            data: data
                .try_into()
                .unwrap_or_else(|_| panic!("Failed to generate history array")),
            pointer: 0,
        }
    }

    /// Insert an item
    pub fn insert(&mut self, item: T) {
        self.data[self.pointer] = Some(item);
        self.pointer += 1;
        if self.pointer > self.data.len() - 1 {
            self.pointer = 0;
        }
    }

    /// Get a specific item from history
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data[index].as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data[index].as_mut()
    }

    /// Get all items from the array in sorted order
    pub fn get_all(&self) -> Vec<&T> {
        let mut result = vec![];
        let mut c = self.pointer;
        for _ in 0..HISTORY_SIZE {
            if let Some(k) = self.data[c].as_ref() {
                result.push(k);
            }

            c += 1;
            if c > self.data.len() - 1 {
                c = 0;
            }
        }

        result
    }

    ///Get the current count of items
    pub fn get_count(&self) -> usize {
        self.pointer
    }
}

#[cfg(test)]
mod test_history {
    use super::*;

    #[test]
    fn insert_get_all() {
        let mut history = History::new();
        for i in 0..10 {
            history.insert(i);
        }
        let items = history.get_all();
        assert_eq!(items.len(), 10);
        for i in 0..10 {
            assert_eq!(*items[i], i);
        }
    }

    #[test]
    fn insert_items() {
        let mut history = History::new();
        for i in 0..50 {
            history.insert(i);
        }
    }

    #[test]
    fn get_items() {
        let mut history = History::new();
        for i in 50..82 {
            history.insert(i as i32);
        }
        for i in 0..32 {
            let g = history.get(i).unwrap();
            assert_eq!(*g as usize, i + 50);
        }
    }

    #[test]
    fn get_items_sorted() {
        let mut history = History::new();
        for i in 0..150 {
            history.insert(i);
        }

        let items = history.get_all();

        let mut prev = i32::MIN;
        for i in items {
            let g = i;
            assert!(prev < *g);
            prev = *g;
        }
    }
}

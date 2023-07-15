// A Vec-backed PriorityQueue
// Probably not the fastest in the world, but it's fun to make a home baked one
//
// 1) Finding the minimum priority (this could actually be cached).
// 2) Finding an already existing item in the priority queue (without knowing the priority),
//    and updating the priority.
// 3) Inserting and removing items.

#[derive(Debug)]
pub struct PriorityQueue<T>(Vec<(usize, T)>);

impl<T> PriorityQueue<T>
where
    T: Clone + PartialEq + Copy,
{
    pub fn new() -> Self {
        PriorityQueue(Vec::new())
    }

    pub fn enqueue(&mut self, item: T, priority: usize) {
        if let Some((index, (stored_priority, item))) = self
            .0
            .iter()
            .enumerate()
            .find(|(_, (_, this_item))| &item == this_item)
        {
            // The item is already in the queue, update the priority if lower
            if priority < *stored_priority {
                self.0[index] = (priority, *item);
            }
        } else {
            // The item is new, add it to the queue
            self.0.push((priority, item))
        }
    }

    pub fn dequeue(&mut self) -> Option<(usize, T)> {
        if let Some((index, _)) = self
            .0
            .iter()
            .enumerate()
            .min_by_key(|(_, (priority, _))| priority)
        {
            let item = self.0[index];
            self.0.swap_remove(index);

            Some(item)
        } else {
            None
        }
    }
}

impl<T> Default for PriorityQueue<T>
where
    T: Clone + PartialEq + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test_gymnastics() {
        let mut pq = PriorityQueue::new();
        pq.enqueue("House", 9);
        pq.enqueue("Shed", 11);
        pq.enqueue("Garage", 1);

        assert_eq!(pq.dequeue(), Some((1, "Garage")));
        assert_eq!(pq.dequeue(), Some((9, "House")));
        assert_eq!(pq.dequeue(), Some((11, "Shed")));
        assert_eq!(pq.dequeue(), None);
    }

    #[test]
    fn changes_priority_if_already_in_queue_and_priority_is_lower() {
        let mut pq = PriorityQueue::new();

        pq.enqueue("House", 9);
        pq.enqueue("House", 1);
        assert_eq!(pq.dequeue(), Some((1, "House")));

        pq.enqueue("House", 2);
        pq.enqueue("House", 3);
        assert_eq!(pq.dequeue(), Some((2, "House")));

        assert_eq!(pq.dequeue(), None);
    }
}

// A Vec-backed PriorityQueue
// Probably not the fastest in the world, but it's fun to make a home baked one

#[derive(Debug)]
pub struct PriorityQueue<T>(Vec<(T, usize)>);

impl<T> PriorityQueue<T>
where
    T: Clone + PartialEq + Copy,
{
    pub fn new() -> Self {
        PriorityQueue(Vec::new())
    }

    pub fn with_one_element(first_element: T, priority: usize) -> Self {
        PriorityQueue(vec![(first_element, priority)])
    }

    pub fn enqueue(&mut self, item: T, priority: usize) {
        if let Some((index, (item, stored_priority))) =
            self.0.iter().enumerate().find(|(_, (this_item, _))| &item == this_item)
        {
            // The item is already in the queue, update the priority if lower
            if priority < *stored_priority {
                self.0[index] = (*item, priority);
            }
        } else {
            // The item is new, add it to the queue
            self.0.push((item, priority))
        }
    }

    pub fn dequeue(&mut self) -> Option<(T, usize)> {
        if let Some((index, _)) = self.0.iter().enumerate().min_by_key(|(_, (_, priority))| priority) {
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

        assert_eq!(pq.dequeue(), Some(("Garage", 1)));
        assert_eq!(pq.dequeue(), Some(("House", 9)));
        assert_eq!(pq.dequeue(), Some(("Shed", 11)));
        assert_eq!(pq.dequeue(), None);
    }

    #[test]
    fn changes_priority_if_already_in_queue_and_priority_is_lower() {
        let mut pq = PriorityQueue::new();

        pq.enqueue("House", 9);
        pq.enqueue("House", 1);
        assert_eq!(pq.dequeue(), Some(("House", 1)));

        pq.enqueue("House", 2);
        pq.enqueue("House", 3);
        assert_eq!(pq.dequeue(), Some(("House", 2)));

        assert_eq!(pq.dequeue(), None);
    }
}

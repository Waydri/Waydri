use std::collections::VecDeque;

use super::window::WindowId;

#[derive(Debug, Clone)]
pub struct WindowGroup {
    pub id: u64,
    pub members: VecDeque<WindowId>,
    pub active: Option<WindowId>,
}

impl WindowGroup {
    pub fn new(id: u64) -> Self {
        WindowGroup {
            id,
            members: VecDeque::new(),
            active: None,
        }
    }

    pub fn add(&mut self, window: WindowId) {
        if !self.members.contains(&window) {
            self.members.push_back(window);
        }
        self.active = Some(window);
    }

    pub fn remove(&mut self, window: WindowId) -> bool {
        let before = self.members.len();
        self.members.retain(|w| *w != window);
        if self.active == Some(window) {
            self.active = self.members.back().copied();
        }
        self.members.len() != before
    }

    pub fn next(&mut self) -> Option<WindowId> {
        if let Some(active) = self.active {
            let mut rotated = VecDeque::new();
            let mut found = false;
            while let Some(front) = self.members.pop_front() {
                if found {
                    rotated.push_back(front);
                    continue;
                }
                if front != active {
                    rotated.push_back(front);
                } else {
                    found = true;
                }
            }
            rotated.push_back(active);
            self.members = rotated;
            self.active = self.members.front().copied();
        }
        self.active
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct GroupRegistry {
    pub groups: Vec<WindowGroup>,
    next_id: u64,
}

impl GroupRegistry {
    pub fn new() -> Self {
        GroupRegistry {
            groups: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.groups.push(WindowGroup::new(id));
        id
    }

    pub fn lookup(&mut self, window: WindowId) -> Option<&mut WindowGroup> {
        self.groups.iter_mut().find(|g| g.members.contains(&window))
    }

    pub fn add_to(&mut self, group_id: u64, window: WindowId) -> bool {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
            group.add(window);
            return true;
        }
        false
    }

    pub fn remove_from_any(&mut self, window: WindowId) {
        for group in self.groups.iter_mut() {
            group.remove(window);
        }
        self.groups.retain(|g| !g.is_empty());
    }
}
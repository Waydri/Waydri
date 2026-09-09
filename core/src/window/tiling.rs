use crate::utils::{Rect, Vec2};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitAxis {
    None,
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitPos {
    First,
    Second,
}

#[derive(Debug)]
pub struct TilingNode {
    pub id: crate::window::WindowId,
    pub children: [Option<Box<TilingNode>>; 2],
    pub split: SplitAxis,
    pub ratio: f32,
}

impl TilingNode {
    pub fn leaf(id: crate::window::WindowId) -> Self {
        TilingNode {
            id,
            children: [None, None],
            split: SplitAxis::None,
            ratio: 0.5,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children[0].is_none() && self.children[1].is_none()
    }

    pub fn insert_leaf(&mut self, position: SplitPos, id: crate::window::WindowId, split: SplitAxis) {
        let index = match position {
            SplitPos::First => 0,
            SplitPos::Second => 1,
        };
        if self.children[index].is_some() {
            return;
        }
        let mut child = TilingNode::leaf(id);
        if self.is_leaf() {
            self.split = split;
        }
        child.split = split;
        self.children[index] = Some(Box::new(child));
    }
}

#[derive(Debug)]
pub struct TilingTree {
    pub root: Option<Box<TilingNode>>,
}

impl Default for TilingTree {
    fn default() -> Self {
        Self::new()
    }
}

impl TilingTree {
    pub fn new() -> Self {
        TilingTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn add(&mut self, id: crate::window::WindowId, split: SplitAxis) {
        match self.root.as_mut() {
            None => {
                self.root = Some(Box::new(TilingNode::leaf(id)));
            }
            Some(root) => {
                root.insert_leaf(SplitPos::Second, id, split);
            }
        }
    }

    pub fn remove(&mut self, id: crate::window::WindowId) -> bool {
        let (retained, found) = remove_from(self.root.take(), id);
        self.root = retained;
        found
    }

    pub fn arrange(
        &self,
        area: Rect,
    ) -> HashMap<crate::window::WindowId, Rect> {
        let mut table = HashMap::new();
        if let Some(root) = &self.root {
            arrange_node(root, area, &mut table);
        }
        table
    }
}

fn remove_from(
    node: Option<Box<TilingNode>>,
    id: crate::window::WindowId,
) -> (Option<Box<TilingNode>>, bool) {
    let Some(mut node) = node else {
        return (None, false);
    };
    if node.id == id && node.is_leaf() {
        return (None, true);
    }
    let (left, left_found) = remove_from(node.children[0].take(), id);
    let (right, right_found) = remove_from(node.children[1].take(), id);
    node.children = [left, right];
    if left_found || right_found {
        if node.is_leaf() {
            return (None, true);
        }
        let only_leaf = node.children[1].is_none() || node.children[0].is_none();
        if only_leaf {
            let surviving = node.children[1].take().or(node.children[0].take());
            return (surviving, true);
        }
        return (Some(node), true);
    }
    (Some(node), false)
}

fn arrange_node(
    node: &TilingNode,
    area: Rect,
    table: &mut HashMap<crate::window::WindowId, Rect>,
) {
    if node.is_leaf() {
        table.insert(node.id, area);
        return;
    }
    let gap_units = 0;
    match node.split {
        SplitAxis::Horizontal => {
            let split_x = area.x + (area.width as f32 * node.ratio) as i32 - gap_units;
            let first = Rect::new(area.x, area.y, (split_x - area.x).max(1), area.height);
            let second = Rect::new(split_x + gap_units, area.y, (area.x + area.width - split_x - gap_units).max(1), area.height);
            if let Some(child) = &node.children[0] {
                arrange_node(child, first, table);
            }
            if let Some(child) = &node.children[1] {
                arrange_node(child, second, table);
            }
        }
        SplitAxis::Vertical => {
            let split_y = area.y + (area.height as f32 * node.ratio) as i32 - gap_units;
            let first = Rect::new(area.x, area.y, area.width, (split_y - area.y).max(1));
            let second = Rect::new(area.x, split_y + gap_units, area.width, (area.y + area.height - split_y - gap_units).max(1));
            if let Some(child) = &node.children[0] {
                arrange_node(child, first, table);
            }
            if let Some(child) = &node.children[1] {
                arrange_node(child, second, table);
            }
        }
        SplitAxis::None => {
            for child in node.children.iter().flatten() {
                arrange_node(child, centered(area), table);
            }
        }
    }
}

fn centered(rect: Rect) -> Rect {
    let center = Vec2::new(rect.center().x, rect.center().y);
    let w = rect.width / 2;
    let h = rect.height / 2;
    Rect::new(center.x as i32 - w / 2, center.y as i32 - h / 2, w, h)
}
use waydri_core::utils::Rect;

fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    Rect::new(x, y, w, h)
}

#[test]
fn window_manager_full_lifecycle() {
    let mut manager = waydri_core::window::WindowManager::new();
    let id = manager.create_window("win".to_string(), "app".to_string(), 0);
    assert_eq!(manager.count(), 1);
    assert!(manager.destroy_window(id));
    assert_eq!(manager.count(), 0);
}

#[test]
fn focus_follows_create_order() {
    let mut manager = waydri_core::window::WindowManager::new();
    let first = manager.create_window("a".to_string(), "app".to_string(), 0);
    let second = manager.create_window("b".to_string(), "app".to_string(), 0);
    assert_eq!(manager.focused(), Some(second));
    manager.focus(first);
    assert_eq!(manager.focused(), Some(first));
}

#[test]
fn windows_are_isolated_by_workspace() {
    let mut manager = waydri_core::window::WindowManager::new();
    manager.create_window("a".to_string(), "app".to_string(), 0);
    manager.create_window("b".to_string(), "app".to_string(), 1);
    assert_eq!(manager.windows_on(0).len(), 1);
    assert_eq!(manager.windows_on(1).len(), 1);
}

#[test]
fn tiling_tree_arranges_rects() {
    use waydri_core::window::{SplitAxis, SplitPos, TilingNode, TilingTree, WindowId};
    let mut tree = TilingTree::new();
    let mut root = TilingNode::leaf(WindowId(0));
    root.insert_leaf(SplitPos::First, WindowId(1), SplitAxis::Horizontal);
    root.insert_leaf(SplitPos::Second, WindowId(2), SplitAxis::Horizontal);
    tree.root = Some(Box::new(root));
    let table = tree.arrange(rect(0, 0, 100, 100));
    assert_eq!(table.len(), 2);
    let first = table.get(&WindowId(1)).copied().unwrap();
    let second = table.get(&WindowId(2)).copied().unwrap();
    assert_eq!(first.width + second.width, 100);
}

#[test]
fn tiling_tree_remove_compacts() {
    use waydri_core::window::{SplitAxis, SplitPos, TilingNode, TilingTree, WindowId};
    let mut tree = TilingTree::new();
    let mut root = TilingNode::leaf(WindowId(0));
    root.insert_leaf(SplitPos::First, WindowId(1), SplitAxis::Vertical);
    root.insert_leaf(SplitPos::Second, WindowId(2), SplitAxis::Vertical);
    tree.root = Some(Box::new(root));
    assert!(tree.remove(WindowId(2)));
    assert!(!tree.is_empty());
    assert!(tree.remove(WindowId(1)));
    assert!(tree.is_empty());
}

#[test]
fn fullscreen_toggle_restores() {
    let mut state = waydri_core::window::FullscreenState::new();
    let id = waydri_core::window::WindowId(5);
    assert!(!state.active());
    assert!(state.toggle(id, rect(0, 0, 200, 100)));
    assert!(state.active());
    assert!(!state.toggle(id, rect(0, 0, 200, 100)));
}

#[test]
fn group_registry_rotates() {
    let mut registry = waydri_core::window::GroupRegistry::new();
    let group_id = registry.create();
    registry.add_to(group_id, waydri_core::window::WindowId(1));
    registry.add_to(group_id, waydri_core::window::WindowId(2));
    if let Some(group) = registry.lookup(waydri_core::window::WindowId(2)) {
        let next = group.next();
        assert!(next.is_some());
    }
}

#[test]
fn tiling_node_leaf_state() {
    let node = waydri_core::window::TilingNode::leaf(waydri_core::window::WindowId(7));
    assert!(node.is_leaf());
    assert_eq!(node.id, waydri_core::window::WindowId(7));
}
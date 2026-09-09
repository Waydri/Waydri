use waydri_core::utils::{Color, Rect};
use waydri_core::window::{
    FullscreenState, TilingTree, WindowGroup, WindowId, WindowManager,
};

fn wid(n: u64) -> WindowId {
    WindowId(n)
}

#[test]
fn create_and_destroy() {
    let mut wm = WindowManager::new();
    let id = wm.create_window("Test".into(), "test.app".into(), 1);
    assert_eq!(wm.count(), 1);
    assert_eq!(wm.focused(), Some(id));
    assert_eq!(wm.destroy_window(id), true);
    assert_eq!(wm.count(), 0);
}

#[test]
fn focus_order() {
    let mut wm = WindowManager::new();
    let a = wm.create_window("A".into(), "a".into(), 1);
    let b = wm.create_window("B".into(), "b".into(), 1);
    assert_eq!(wm.focused(), Some(b));
    wm.focus(a);
    assert_eq!(wm.focused(), Some(a));
    assert_eq!(wm.window(a).unwrap().title, "A");
}

#[test]
fn workspace_isolation() {
    let mut wm = WindowManager::new();
    wm.create_window("W1".into(), "app".into(), 1);
    wm.create_window("W2".into(), "app".into(), 2);
    assert_eq!(wm.windows_on(1).len(), 1);
    assert_eq!(wm.windows_on(2).len(), 1);
}

#[test]
fn visible_sorted_by_depth() {
    let mut wm = WindowManager::new();
    let a = wm.create_window("A".into(), "a".into(), 1);
    let b = wm.create_window("B".into(), "b".into(), 1);
    assert_eq!(wm.visible_sorted(), vec![a, b]);
    wm.focus(a);
    assert_eq!(wm.visible_sorted(), vec![a, b], "depth ordering is stable");
}

#[test]
fn placement_and_alpha() {
    let mut wm = WindowManager::new();
    let id = wm.create_window("W".into(), "app".into(), 1);
    wm.set_placement(id, Rect::new(0, 0, 50, 50));
    assert_eq!(wm.window(id).unwrap().placement.width, 50);
    wm.set_alpha(id, 0.5);
    assert_eq!(wm.window(id).unwrap().alpha, 0.5);
}

#[test]
fn window_buffer_carries_size() {
    let mut wm = WindowManager::new();
    let id = wm.create_window("W".into(), "app".into(), 1);
    let buffer = vec![0u8; 16 * 16 * 4];
    wm.window_mut(id).unwrap().set_buffer(buffer.clone(), 16, 16);
    assert_eq!(wm.window(id).unwrap().surface_size(), (16, 16));
}

#[test]
fn window_color_default() {
    let mut wm = WindowManager::new();
    let id = wm.create_window("W".into(), "app".into(), 1);
    let color = wm.window(id).unwrap().color;
    assert_eq!(color.to_hex(), Color::from_hex(0x1E1E28).to_hex());
}

#[test]
fn ids_enumeration() {
    let mut wm = WindowManager::new();
    let a = wm.create_window("A".into(), "a".into(), 1);
    let b = wm.create_window("B".into(), "b".into(), 1);
    let mut ids = wm.ids();
    ids.sort_by_key(|id| id.0);
    assert_eq!(ids, vec![a, b]);
}

#[test]
fn tiling_tree_add_remove() {
    use waydri_core::window::tiling::{SplitAxis, SplitPos, TilingNode};
    let mut tree = TilingTree::new();
    assert!(tree.is_empty());
    let mut root = TilingNode::leaf(wid(0));
    root.insert_leaf(SplitPos::First, wid(1), SplitAxis::Horizontal);
    root.insert_leaf(SplitPos::Second, wid(2), SplitAxis::Horizontal);
    tree.root = Some(Box::new(root));
    assert!(!tree.is_empty());
    assert_eq!(tree.arrange(Rect::new(0, 0, 100, 100)).len(), 2);
    assert_eq!(tree.remove(wid(1)), true);
    assert_eq!(tree.remove(wid(99)), false);
}

#[test]
fn fullscreen_toggle() {
    let mut state = FullscreenState::new();
    let rect = Rect::new(0, 0, 400, 400);
    assert!(!state.active());
    state.enter(wid(1), rect);
    assert!(state.active());
    let (id, restored) = state.exit().unwrap();
    assert_eq!(id, wid(1));
    assert_eq!(restored, rect);
}

#[test]
fn window_group_rotation() {
    let mut group = WindowGroup::new(1);
    group.add(wid(1));
    group.add(wid(2));
    group.add(wid(3));
    assert_eq!(group.len(), 3);
    let _ = group.next();
    assert!(group.active.is_some());
    group.remove(wid(2));
    assert_eq!(group.len(), 2);
}
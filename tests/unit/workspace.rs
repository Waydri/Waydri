use waydri_core::window::WindowId;
use waydri_core::workspace::{
    PersistentWorkspace, PersistentWorkspaces, SpecialWorkspace, WorkspaceManager,
};

fn wid(n: u64) -> WindowId {
    WindowId(n)
}

#[test]
fn default_workspaces() {
    let manager = WorkspaceManager::new(5);
    assert_eq!(manager.count(), 5);
    assert_eq!(manager.active_index(), 0);
    assert_eq!(manager.active_workspace().name, "workspace 1");
}

#[test]
fn switching() {
    let mut manager = WorkspaceManager::new(3);
    assert_eq!(manager.switch(2), true);
    assert_eq!(manager.active_index(), 2);
    assert_eq!(manager.next(), 0);
    assert_eq!(manager.active_index(), 0);
    assert_eq!(manager.previous(), 2);
}

#[test]
fn switch_out_of_range_rejected() {
    let mut manager = WorkspaceManager::new(3);
    assert_eq!(manager.switch(9), false);
    assert_eq!(manager.active_index(), 0);
}

#[test]
fn add_remove_windows() {
    let mut manager = WorkspaceManager::new(3);
    let id = wid(42);
    manager.add_window(id);
    assert_eq!(manager.active_workspace().window_ids(), vec![id]);
    assert_eq!(manager.move_window(id, 1), true);
    assert_eq!(manager.switch(1), true);
    assert!(manager.active_workspace().has_window(id));
}

#[test]
fn layout_tracking() {
    let mut manager = WorkspaceManager::new(3);
    manager.set_layout("grid");
    assert_eq!(manager.layout_name(), "grid");
    assert_eq!(manager.active_workspace().layout_name(), "grid");
}

#[test]
fn visibility_toggle() {
    let mut manager = WorkspaceManager::new(3);
    assert_eq!(manager.toggle_workspace_visibility(0), false);
    assert_eq!(manager.active_workspace().visible, false);
    assert_eq!(manager.toggle_workspace_visibility(0), true);
    assert_eq!(manager.active_workspace().visible, true);
}

#[test]
fn special_workspace_lifecycle() {
    let mut special = SpecialWorkspace::new(90, "scratch".into());
    assert!(special.is_empty());
    let id = wid(7);
    special.add_window(id);
    assert_eq!(special.window_ids(), vec![id]);
    assert!(!special.is_empty());
    assert_eq!(special.toggle(), true);
    assert!(special.visible);
    special.toggle();
    assert!(!special.visible);
}

#[test]
fn persistent_round_trip() {
    let path = "/tmp/waydri_ws_test.json";
    let mut workspaces = PersistentWorkspaces::new(path.into());
    workspaces.push(PersistentWorkspace::new(1, "main".into(), "master_stack".into()));
    workspaces.push(PersistentWorkspace::new(2, "code".into(), "dwindle".into()));
    workspaces.save().unwrap();
    let loaded = workspaces.load();
    assert_eq!(loaded.len(), 2);
    assert_eq!(workspaces.get(2).unwrap().name, "code");
    assert_eq!(PersistentWorkspace::parse("3:term:grid").unwrap().id, 3);
    let _ = std::fs::remove_file(path);
}

#[test]
fn move_window_to_missing_target() {
    let mut manager = WorkspaceManager::new(3);
    let id = wid(3);
    assert_eq!(manager.move_window(id, 8), false);
}

#[test]
fn workspace_sets_layout_fluently() {
    let ws = waydri_core::workspace::Workspace::new(1, "one".into()).with_layout("grid");
    assert_eq!(ws.layout_name(), "grid");
}
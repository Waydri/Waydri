use waydri_core::workspace::{PersistentWorkspace, SpecialWorkspace, Workspace, WorkspaceManager};
use waydri_core::window::WindowId;

#[test]
fn workspace_manager_switches_active() {
    let mut manager = WorkspaceManager::new(4);
    assert_eq!(manager.active_index(), 0);
    assert!(manager.switch(3));
    assert_eq!(manager.active_index(), 3);
}

#[test]
fn workspace_next_wraps() {
    let mut manager = WorkspaceManager::new(3);
    manager.next();
    manager.next();
    manager.next();
    assert_eq!(manager.active_index(), 0);
}

#[test]
fn move_window_between_workspaces() {
    let mut manager = WorkspaceManager::new(3);
    let id = WindowId(1);
    manager.add_window(id);
    assert!(manager.move_window(id, 2));
    assert!(manager.workspace(2).map(|w| w.has_window(id)).unwrap_or(false));
}

#[test]
fn persistent_workspace_round_trips() {
    let workspace = PersistentWorkspace::new(2, "code".to_string(), "dwindle".to_string());
    let serialized = workspace.serialize();
    let parsed = PersistentWorkspace::parse(&serialized).unwrap();
    assert_eq!(parsed.id, 2);
    assert_eq!(parsed.name, "code");
    assert_eq!(parsed.layout_name, "dwindle");
}

#[test]
fn special_workspace_toggles() {
    let mut special = SpecialWorkspace::new(9, "scratch".to_string());
    assert!(special.toggle());
    special.add_window(WindowId(4));
    assert_eq!(special.window_ids(), vec![WindowId(4)]);
    assert!(!special.toggle());
}

#[test]
fn workspace_layout_change() {
    let mut workspace = Workspace::new(1, "main".to_string());
    workspace.set_layout("grid");
    assert_eq!(workspace.layout_name(), "grid");
}
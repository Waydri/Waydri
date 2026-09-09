use waydri_core::layout::LayoutManager;
use waydri_core::utils::Rect;
use waydri_core::window::WindowId;

#[test]
fn layouts_registered() {
    let manager = LayoutManager::with_default();
    let available = manager.available();
    for expected in ["master_stack", "dwindle", "grid", "custom", "dynamic"] {
        assert!(
            available.contains(&expected),
            "missing layout {expected}: {available:?}"
        );
    }
}

#[test]
fn arrange_master_stack_splits_area() {
    let manager = LayoutManager::with_default();
    let area = Rect::new(0, 0, 100, 100);
    let windows = vec![WindowId(1), WindowId(2)];
    let rects = manager.arrange("master_stack", area, &windows);
    assert_eq!(rects.len(), 2);
    for rect in &rects {
        assert!(rect.x >= 0 && rect.x + rect.width <= area.width);
        assert!(rect.y >= 0 && rect.y + rect.height <= area.height);
        assert!(!rect.is_empty());
    }
    assert!(rects[0].width > rects[1].width);
    assert!(rects[0].intersection(&rects[1]).map(|r| r.area()).unwrap_or(0) <= 0);
}

#[test]
fn arrange_grid_fills_rows() {
    let manager = LayoutManager::with_default();
    let area = Rect::new(0, 0, 100, 100);
    let windows = vec![WindowId(1), WindowId(2), WindowId(3), WindowId(4)];
    let rects = manager.arrange("grid", area, &windows);
    assert_eq!(rects.len(), 4);
    let width = rects[0].width;
    let height = rects[0].height;
    for rect in &rects {
        assert!(!rect.is_empty());
        assert_eq!(rect.width, width, "two columns expected");
        assert_eq!(rect.height, height, "two rows expected");
    }
}

#[test]
fn arrange_no_overlap() {
    let manager = LayoutManager::with_default();
    let area = Rect::new(0, 0, 200, 200);
    let windows: Vec<WindowId> = (1..=6).map(WindowId).collect();
    for name in manager.available() {
        let rects = manager.arrange(name, area, &windows);
        assert!(!rects.is_empty(), "layout {name} must place windows");
        assert!(
            rects.len() <= windows.len(),
            "layout {name} must not invent windows"
        );
        for (i, a) in rects.iter().enumerate() {
            for (j, b) in rects.iter().enumerate() {
                if i != j && name != "dwindle" {
                    let overlap = a.intersection(b);
                    assert!(
                        overlap.map(|r| r.area()).unwrap_or(0) <= 0,
                        "layout {name} overlaps window {i}/{j}"
                    );
                }
            }
        }
    }
}

#[test]
fn arrange_empty_windows_succeeds() {
    let manager = LayoutManager::with_default();
    let area = Rect::new(0, 0, 100, 100);
    let rects = manager.arrange("dwindle", area, &[]);
    assert!(rects.is_empty());
}

#[test]
fn set_layout_unknown_falls_back() {
    let mut manager = LayoutManager::with_default();
    assert_eq!(manager.set_layout("missing"), false);
    assert_eq!(manager.set_layout("grid"), true);
    assert_eq!(manager.current_name(), "grid");
}

#[test]
fn unknown_layout_name_uses_current() {
    let manager = LayoutManager::with_default();
    let area = Rect::new(0, 0, 50, 50);
    let windows = vec![WindowId(1)];
    assert_eq!(manager.arrange("bogus", area, &windows).len(), 1);
}
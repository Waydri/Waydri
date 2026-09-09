use waydri_core::wayland::primary_selection::{PrimarySelection, SelectionOffer, clipboard_to_offers};

#[test]
fn offer_supports_mime() {
    let offer = SelectionOffer::new(3, 1, vec!["text/plain".to_string(), "text/html".to_string()]);
    assert!(offer.supports("text/plain"));
    assert!(!offer.supports("image/png"));
}

#[test]
fn primary_selection_set_and_clear() {
    let mut selection = PrimarySelection::new();
    assert!(selection.active_offer().is_none());
    selection.set(SelectionOffer::new(1, 1, vec!["text/plain".to_string()]));
    assert!(selection.active_offer().is_some());
    selection.clear();
    assert!(selection.active_offer().is_none());
}

#[test]
fn clipboard_offer_describes_text() {
    let offer = clipboard_to_offers("hello", 7);
    assert!(offer.supports("text/plain"));
    assert_eq!(offer.client_id, 7);
}

#[test]
fn selection_activating_last() {
    let mut selection = PrimarySelection::new();
    selection.set(SelectionOffer::new(1, 1, vec!["a".to_string()]));
    selection.set(SelectionOffer::new(2, 2, vec!["b".to_string()]));
    let active = selection.active_offer().unwrap();
    assert_eq!(active.source_id, 2);
}

#[test]
fn data_offer_accepts_mime() {
    let selection = SelectionOffer::new(1, 1, vec!["text/plain".to_string()]);
    let mut offer = waydri_core::wayland::data_device::DataOffer::from_selection(9, &selection);
    assert!(offer.accept(0));
    assert!(!offer.accept(5));
}
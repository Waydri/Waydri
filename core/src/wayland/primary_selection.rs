use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionOffer {
    pub mime_types: Vec<String>,
    pub source_id: u32,
    pub client_id: u32,
}

impl SelectionOffer {
    pub fn new(source_id: u32, client_id: u32, mime_types: Vec<String>) -> Self {
        SelectionOffer {
            mime_types,
            source_id,
            client_id,
        }
    }

    pub fn supports(&self, mime: &str) -> bool {
        self.mime_types.iter().any(|m| m == mime)
    }
}

#[derive(Debug, Clone)]
pub struct PrimarySelection {
    pub offers: Vec<SelectionOffer>,
    pub active: Option<usize>,
}

impl Default for PrimarySelection {
    fn default() -> Self {
        PrimarySelection::new()
    }
}

impl PrimarySelection {
    pub fn new() -> Self {
        PrimarySelection {
            offers: Vec::new(),
            active: None,
        }
    }

    pub fn set(&mut self, offer: SelectionOffer) {
        self.offers.push(offer);
        self.active = Some(self.offers.len() - 1);
    }

    pub fn clear(&mut self) {
        self.offers.clear();
        self.active = None;
    }

    pub fn active_offer(&self) -> Option<&SelectionOffer> {
        self.active.and_then(|index| self.offers.get(index))
    }
}

pub fn clipboard_to_offers(text: &str, client_id: u32) -> SelectionOffer {
    let mime_types = vec!["text/plain".to_string()];
    let mut offer = SelectionOffer::new(0, client_id, mime_types);
    offer.mime_types.push("text/plain;charset=utf-8".to_string());
    let _ = text;
    offer
}
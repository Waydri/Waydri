use super::primary_selection::SelectionOffer;

#[derive(Debug, Clone)]
pub struct DataOffer {
    pub offer_id: u32,
    pub mime_types: Vec<String>,
    pub accepted: Option<usize>,
}

impl DataOffer {
    pub fn from_selection(offer_id: u32, selection: &SelectionOffer) -> Self {
        DataOffer {
            offer_id,
            mime_types: selection.mime_types.clone(),
            accepted: None,
        }
    }

    pub fn accept(&mut self, mime_index: usize) -> bool {
        if mime_index < self.mime_types.len() {
            self.accepted = Some(mime_index);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataDevice {
    pub client_id: u32,
    pub offers: Vec<DataOffer>,
    pub selection: Option<u32>,
}

impl DataDevice {
    pub fn new(client_id: u32) -> Self {
        DataDevice {
            client_id,
            offers: Vec::new(),
            selection: None,
        }
    }

    pub fn add_offer(&mut self, offer: DataOffer) {
        self.offers.push(offer);
    }

    pub fn set_selection(&mut self, offer_id: u32) {
        self.selection = Some(offer_id);
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn selected(&self) -> Option<&DataOffer> {
        self.selection
            .and_then(|id| self.offers.iter().find(|offer| offer.offer_id == id))
    }
}
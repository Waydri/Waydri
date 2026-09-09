#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistentWorkspace {
    pub id: u64,
    pub name: String,
    pub layout_name: String,
}

impl PersistentWorkspace {
    pub fn new(id: u64, name: String, layout_name: String) -> Self {
        PersistentWorkspace {
            id,
            name,
            layout_name,
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}:{}:{}", self.id, self.name, self.layout_name)
    }

    pub fn parse(line: &str) -> Option<PersistentWorkspace> {
        let mut parts = line.splitn(3, ':');
        let id = parts.next()?.parse().ok()?;
        let name = parts.next()?.to_string();
        let layout_name = parts.next()?.to_string();
        Some(PersistentWorkspace::new(id, name, layout_name))
    }
}

pub struct PersistentWorkspaces {
    pub workspaces: Vec<PersistentWorkspace>,
    pub path: String,
}

impl PersistentWorkspaces {
    pub fn new(path: String) -> Self {
        PersistentWorkspaces {
            workspaces: Vec::new(),
            path,
        }
    }

    pub fn push(&mut self, workspace: PersistentWorkspace) {
        self.workspaces.push(workspace);
    }

    pub fn get(&self, id: u64) -> Option<&PersistentWorkspace> {
        self.workspaces.iter().find(|w| w.id == id)
    }

    pub fn load(&self) -> Vec<PersistentWorkspace> {
        let contents = std::fs::read_to_string(&self.path).unwrap_or_default();
        contents
            .lines()
            .filter_map(PersistentWorkspace::parse)
            .collect()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut out = String::new();
        for workspace in &self.workspaces {
            out.push_str(&workspace.serialize());
            out.push('\n');
        }
        std::fs::write(&self.path, out)
    }
}

use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteLinks {
    pub path: String,
    pub href: String,
    pub disp: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VecSiteLinks {
    pub links: Vec<SiteLinks>,
}

impl VecSiteLinks {

    pub fn query_results(&self) -> VecSiteLinks {
        self.clone()
    }

    pub fn add_vec(&mut self, site_links: SiteLinks) -> Self {
        self.links.push(site_links);
        self.clone()
    }

    pub fn remove_vec(&mut self, index: usize) -> Self {
        self.links.remove(index);
        self.clone()
    }

}
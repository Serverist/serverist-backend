pub enum OrderBy {
    CreatedAt,
    UpdatedAt,
}

pub enum Sort {
    Asc,
    Desc
}

pub struct Page {
    pub offset: u32,
    pub limit: u16
}
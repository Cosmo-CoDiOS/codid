#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CoDiContactNumber {
    pub(crate) phone_type: String,
    pub(crate) number: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CoDiContact {
    pub(crate) name: String,
    pub(crate) phone: Vec<CoDiContactNumber>,
}

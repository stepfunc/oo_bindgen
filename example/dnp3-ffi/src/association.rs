use dnp3::master::handle::AssociationHandle;

pub struct Association {
    pub runtime: *mut tokio::runtime::Runtime,
    pub handle: AssociationHandle,
}

pub unsafe fn association_destroy(association: *mut Association) {
    if !association.is_null() {
        let association = Box::from_raw(association);
        let runtime = association.runtime.as_mut().unwrap();
        runtime.block_on(association.handle.remove());
    }
}

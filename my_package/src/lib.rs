//---------------------------------------------------------------------------------------------
// INFO: VISIBILITY SPECIFIERS
// INFO: PRODUCT Module
// 1. pub
// 2. pub(crate) => Only accessible inside the current crate
// 3. pub(self) => Only accessible within the same module - default behavior
// 4. pub(super) => Only accessible within the parent module

// --------------------------------------------------------------------------------------------
// INFO: PRODUCT Module
mod product;

// INFO: CUSTOMER Module
mod customer;

// INFO: ORDER Module
mod order;

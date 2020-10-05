#[macro_use]
extern crate assemblylift_core_iomod_guest;

export_iomod_guest!(akkoro, std, crypto);

call!(uuid4, () => String);

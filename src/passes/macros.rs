// try! macro for passes
// produces an error (pass can be executed further)
// expects `self` to have a `result` field of type `::passes::results::PassResultReference`
// usage: `pass_try!(self, some_expression)`
macro_rules! pass_try {
    ($s:expr, $e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => {
            $s.result.borrow_mut().add_error(Box::new(err));
            return;
        },
    });
}

// try! macro for passes
// produces a fatal error (pass has to stop)
// expects `self` to have a `result` field of type `::passes::results::PassResultReference`
// usage: `pass_try_fatal!(self, some_expression)`
macro_rules! pass_try_fatal {
    ($s:expr, $e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => {
            $s.result.borrow_mut().add_fatal_error(Box::new(err));
            return;
        },
    });
}

macro_rules! pass_err {
    ($s:expr, $w:expr) => (
        $s.result.borrow_mut().add_error($w);
    );
}

// borrows the symbol table
// expects `self` to have a `symbol_table` field of type `::type_system::symbol_table::SymbolTableReference`
// usage: `symbol_table!(self)`
macro_rules! symbol_table {
    ($s:expr) => (
        $s.symbol_table.borrow()
    );
}

// mutably borrows the symbol table
// expects `self` to have a `symbol_table` field of type `::type_system::symbol_table::SymbolTableReference`
// usage: `symbol_table_mut!(self)`
macro_rules! symbol_table_mut {
    ($s:expr) => (
        $s.symbol_table.borrow_mut()
    );
}

// borrows the pass result container
// expects `self` to have a `result` field of type `::passes::results::PassResultReference`
// usage: `result!(self)`
macro_rules! result {
    ($s:expr) => (
        $s.result.borrow()
    );
}

// mutably borrows the pass result container
// expects `self` to have a `result` field of type `::passes::results::PassResultReference`
// usage: `result_mut!(self)`
macro_rules! result_mut {
    ($s:expr) => (
        $s.result.borrow_mut()
    );
}

// creates a simple struct implementing `::new(...)` and `AstWalker` without any custom struct members
macro_rules! ast_pass {
    ($name:ident, $body:tt) => (
        pub struct $name {
            symbol_table: SymbolTableReference,
            result: PassResultReference,
        }

        impl $name {
            pub fn new(symbol_table: SymbolTableReference, result: PassResultReference) -> $name {
                $name {
                    symbol_table: symbol_table,
                    result: result,
                }
            }
        }

        impl AstWalker for $name $body
    );
}

macro_rules! ast_pass_impl {
    ($name:ident, $body:tt) => (
        impl AstWalker for $name $body
    );
}

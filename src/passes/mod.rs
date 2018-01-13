pub mod ast;
pub mod mir;

#[derive(Debug)]
pub enum PassError {
    Warnings,
    Errors,
    Fatal,
}

pub type PassResult = Result<(), PassError>;

// TODO
impl ::std::convert::From<::type_system::error::TypeError> for PassError {
    fn from(other: ::type_system::error::TypeError) -> Self {
        PassError::Fatal
    }
}

pub trait Pass<T> {
    fn execute(&mut self, items: &mut T) -> PassResult;
}

pub struct PassSystem<T> {
    passes: Vec<Box<Pass<T>>>,
}

impl<T> PassSystem<T> {
    pub fn new() -> PassSystem<T> {
        PassSystem {
            passes: Vec::new(),
        }
    }

    pub fn add_pass(&mut self, pass: Box<Pass<T>>) {
        self.passes.push(pass);
    }

    pub fn execute(&mut self, items: &mut T) {
        for pass in self.passes.iter_mut() {
            match pass.execute(items) {
                Ok(()) => (),
                Err(e) => match e {
                    PassError::Warnings => (),
                    PassError::Errors => (),
                    PassError::Fatal => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExamplePass {
        pub executed: bool,
    }

    impl ExamplePass {
        pub fn new() -> ExamplePass {
            ExamplePass {
                executed: false,
            }
        }
    }

    impl<usize> Pass<usize> for ExamplePass {
        fn execute(&mut self, items: &mut usize) -> PassResult {
            self.executed = true;
            Ok(())
        }
    }

    #[test]
    pub fn it_executes_all_passes() {
        let pass_one = ExamplePass::new();
        let pass_two = ExamplePass::new();

        let mut pass_system = PassSystem::new();

        pass_system.add_pass(Box::new(pass_one));
        pass_system.add_pass(Box::new(pass_two));

        pass_system.execute(&mut 0);

        // TODO no idea how to assert this
    }
}

pub mod ast;
pub mod mir;

pub enum PassError {
    Warnings,
    Errors,
    Fatal,
}

pub type PassResult = Result<(), PassError>;

pub trait Pass<T> {
    fn execute(&mut self, items: &mut T);
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
            pass.execute(items);
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
        fn execute(&mut self, items: &mut usize) {
            self.executed = true;
        }
    }

    #[test]
    pub fn it_executes_all_passes() {
        let mut pass_one = ExamplePass::new();
        let mut pass_two = ExamplePass::new();

        let mut pass_system = PassSystem::new();

        pass_system.add_pass(Box::new(pass_one));
        pass_system.add_pass(Box::new(pass_two));

        pass_system.execute(&mut 0);

        // TODO no idea how to assert this
    }
}

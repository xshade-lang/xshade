#[macro_use]
pub mod macros;
pub mod results;
pub mod ast;
pub mod error;

pub trait Pass<T> {
    fn execute(&mut self, items: &mut T);
}

pub struct PassCollection<T> {
    passes: Vec<Box<Pass<T>>>,
}

impl<T> PassCollection<T> {
    pub fn new() -> PassCollection<T> {
        PassCollection {
            passes: Vec::new(),
        }
    }

    pub fn from_passes(passes: Vec<Box<Pass<T>>>) -> PassCollection<T> {
        PassCollection {
            passes: passes,
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
    use ::std::cell::RefCell;
    use ::std::rc::Rc;

    struct ExamplePass {
        executions: Rc<RefCell<usize>>,
    }

    impl ExamplePass {
        pub fn new(executions: Rc<RefCell<usize>>) -> ExamplePass {
            ExamplePass {
                executions: executions,
            }
        }
    }

    impl<usize> Pass<usize> for ExamplePass {
        fn execute(&mut self, items: &mut usize) {
            *self.executions.try_borrow_mut().unwrap() += 1;
        }
    }

    #[test]
    pub fn it_executes_all_passes() {
        let executions = Rc::new(RefCell::new(0));
        let pass_one = ExamplePass::new(Rc::clone(&executions));
        let pass_two = ExamplePass::new(Rc::clone(&executions));

        let mut pass_system = PassCollection::new();

        pass_system.add_pass(Box::new(pass_one));
        pass_system.add_pass(Box::new(pass_two));

        pass_system.execute(&mut 0);

        assert_eq!(2, *executions.try_borrow_mut().unwrap());
    }
}

use std::collections::HashMap;

pub type StringId = usize;

#[derive(Default)]
pub struct StringInterner {
	storage : HashMap<StringId, String>,
	index : StringId,
}

impl StringInterner {
	fn find(&self, id : StringId) -> Option<String> {
		let tmp: Option<&String> = self.storage.get(&id);
		match tmp {
			Some(s) => Some(s.clone()),
			None => None
		}
	}
	
	fn intern(&mut self, s: &str) -> StringId {
		let not_already_contained : bool = self.storage.values().filter(|&val| *val == s).count() == 0;
		if not_already_contained { 
	        self.index += 1;
		    self.storage.insert(self.index, s.to_owned());
		   
		    self.index
		} else {
			0
		}
	}
	
}

fn print_interner(interner : &mut StringInterner) {
	println!("Printing current interner values:");
	for (key, value) in &interner.storage {
        println!("  Id: {} / Value: {}", key, value);
    }
	println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn move_around_test(interner: &mut StringInterner) -> StringId {
        interner.intern("fugidawubidth")
    }

    #[test]
    fn basic_insert_read() {
        let mut interner: StringInterner = Default::default();
		
		let id0 : StringId = interner.intern("Test");
		let id1 : StringId = interner.intern("Test2");	
		
		assert_eq!(id0, 1);
		assert_eq!(id1, 2);
		
		let s0 : Option<String> = interner.find(id0);
		let s1 : Option<String> = interner.find(id1);	

		assert_eq!(s0, Some(String::from("Test")));
		assert_eq!(s1, Some(String::from("Test2")));
    }
	
	#[test]
	fn insert_duplicate() {
		let mut interner: StringInterner = Default::default();
		
		let id0 : StringId = interner.intern("Test");
		let id1 : StringId = interner.intern("Test");	
		
		assert_eq!(id0, 1);
		assert_eq!(id1, 0);
		
		let s0 : Option<String> = interner.find(id0);
		let s1 : Option<String> = interner.find(id1);	
		
		assert_eq!(s0, Some(String::from("Test")));
		assert_eq!(s1, None);
	}
	
	#[test]
	fn insert_in_subroutine() {
		let mut interner: StringInterner = Default::default();
		let id0 : StringId = interner.intern("Test");
		let id1 : StringId = interner.intern("Test2");	
		
		assert_eq!(id0, 1);
		assert_eq!(id1, 2);
		
		let s0 : Option<String> = interner.find(id0);
		let s1 : Option<String> = interner.find(id1);	
		
		assert_eq!(s0, Some(String::from("Test")));
		assert_eq!(s1, Some(String::from("Test2")));
		
		let id2 : StringId = move_around_test(&mut interner);
		assert_eq!(id2, 3);
		
		let s2 : Option<String> = interner.find(id2);
		assert_eq!(s2, Some(String::from("fugidawubidth")));
	}
}

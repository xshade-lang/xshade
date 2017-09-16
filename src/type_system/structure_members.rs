use ::type_system::type_environment::TypeReference;

#[derive(Debug, Eq, PartialEq)]
pub struct StructureMember {
    pub member_name: String,
    pub member_type: TypeReference,
}

impl StructureMember {
    pub fn new(member_name: String, member_type: TypeReference) -> StructureMember {
        StructureMember {
            member_name: member_name,
            member_type: member_type,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructureMembers {
    members: Vec<StructureMember>,
}

impl StructureMembers {
    pub fn new(members: Vec<StructureMember>) -> StructureMembers {
        StructureMembers {
            members: members,
        }
    }

    pub fn find_member_type(&self, member_name: &str) -> Option<TypeReference> {
        for m in self.members.iter() {
            if m.member_name == member_name {
                return Some(m.member_type);
            }
        }
        None
    }

    pub fn is_assignable_with(&self, members: &Vec<StructureMember>) -> bool {
        if members.len() != self.members.len() {
            return false;
        }

        for a in self.members.iter() {
            let mut found = false;
            for b in members.iter() {
                if a == b { found = true; break; }
            }

            if !found {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn members_are_equal() {
        let a = vec![StructureMember::new("a".to_string(), TypeReference::new(0)), StructureMember::new("b".to_string(), TypeReference::new(1))];
        let b = vec![StructureMember::new("a".to_string(), TypeReference::new(0)), StructureMember::new("b".to_string(), TypeReference::new(1))];

        let struct_members = StructureMembers::new(a);

        assert!(struct_members.is_assignable_with(b));
    }

    #[test]
    fn unordered_members_are_equal() {
        let a = vec![StructureMember::new("a".to_string(), TypeReference::new(0)), StructureMember::new("b".to_string(), TypeReference::new(1))];
        let b = vec![StructureMember::new("b".to_string(), TypeReference::new(1)), StructureMember::new("a".to_string(), TypeReference::new(0))];

        let struct_members = StructureMembers::new(a);

        assert!(struct_members.is_assignable_with(b));
    }

    #[test]
    fn unordered_unequal_members_are_not_equal() {
        let a = vec![StructureMember::new("a".to_string(), TypeReference::new(1)), StructureMember::new("b".to_string(), TypeReference::new(1))];
        let b = vec![StructureMember::new("b".to_string(), TypeReference::new(1)), StructureMember::new("a".to_string(), TypeReference::new(0))];

        let struct_members = StructureMembers::new(a);

        assert!(!struct_members.is_assignable_with(b));
    }
}

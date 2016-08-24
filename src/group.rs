use candidate::*;

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub candidate_ids: Vec<CandidateId>,
}

pub fn get_group_list(candidates: &[Candidate], state: &str) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    for c in candidates.iter().filter(|c| c.state == state) {
        // If there's already a group for this candidate, add them and continue.
        if let Some(current_group) = groups.last_mut() {
            if current_group.name == c.group_name {
                current_group.candidate_ids.push(c.id);
                continue;
            }
        }
        // Otherwise, push a new group to the list (skipping the ungrouped group).
        if c.group_name != "UG" {
            groups.push(Group { name: c.group_name.clone(), candidate_ids: vec![c.id] });
        }
    }
    groups
}

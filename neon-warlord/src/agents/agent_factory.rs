//! Instantiates agents

use crate::agents::agentd_definitions::get_agent_0_definition;
use regex::Regex;

pub struct AgentFactory {
    re: Regex,
}

impl AgentFactory {
    pub fn new() -> Self {
        let re = Regex::new(r"(?P<id>\d+)(?:-(?P<kind>[A-Z])(?P<target>\d+))?\s").unwrap();

        Self { re  }
    }

    pub fn create_agent<const NR_SLICES: usize, const R: usize, const C: usize>(
        &self,
        layers: &[[[&'static str; C]; R]; NR_SLICES],
    ) -> Vec<Node> {
        let mut nodes = Vec::new();

        for nr_slice in 0..NR_SLICES {
            for r in 0..R {
                for c in 0..C {
                    let content = layers[nr_slice][r][c];
                    let mut elem = self.parse(content);
                    elem.location = (nr_slice, r, c);

                    nodes.push(elem);
                }
            }
        }

        nodes.sort_by_key(|node| node.id);

        nodes
    }

    fn parse(&self, elem: &str) -> Node {
        let caps = self
            .re
            .captures(elem)
            .expect("regex error, failed parsing agent definition");

        let id = &caps["id"];
        let kind = &caps["kind"];
        let target = &caps["target"];

        let id: u32 = id.parse().unwrap();
        let link_target: u32 = target.parse().unwrap();

        let link_kind = match kind {
            "F" => LinkKind::fixed,
            "L" => LinkKind::linked,
            "S" => LinkKind::sticky,
            "O" => LinkKind::origin,
            &_ => panic!("Error parsing agent definition")
        };

        Node {
            id,
            link_kind,
            link_target,
            location: (0, 0, 0),
        }
    }
}

enum LinkKind {
    /// Node is fixed in location to the other node
    fixed,
    /// Node is fixed in distance to the other node
    linked,
    /// Node is fixed in distance to the other node and is sticky to the ground
    sticky,
    /// Treated as origin of the structure (has no parent links)
    origin,
}

pub struct Node {
    id: u32,
    link_kind: LinkKind,
    link_target: u32,
    location: (usize, usize, usize)
}


fn main_func()
{

    let definition = get_agent_0_definition();

    // create_agent(&definition);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_agent_definition() {
        let factory = AgentFactory::new();

        let elem = "13-S9 ";

        let caps = factory
            .re
            .captures(elem)
            .expect("regex should match");

        assert_eq!(&caps["id"], "13");
        assert_eq!(&caps["kind"], "S");
        assert_eq!(&caps["target"], "9");
    }
}

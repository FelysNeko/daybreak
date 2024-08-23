import interface

node_template = '''
#[derive(Clone)]
pub struct {node_type} {{
    todo!()
}}

impl Debug for {node_type} {{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
        todo!()
    }}
}}

impl From<CacheResult> for Option<{node_type}> {{
    fn from(value: CacheResult) -> Self {{
        match value {{
            CacheResult::{node_type}(inner) => inner,
            _ => panic!("cache not matched")
        }}
    }}
}}
'''


main, types = interface.generate('rspegen.gram', False)

fmt_node_list = [node_template.format(node_type=each) for each in types]
nodes = '\n'.join(fmt_node_list)

print(main)
print(nodes)
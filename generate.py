import interface

path = 'rspegen.gram'
header = f'// Generated from {path} by generate.py\n'

node_template = '''
#[derive(Clone)]
pub struct {nt} {{
    todo!()
}}

impl Debug for {nt} {{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
        todo!()
    }}
}}

impl From<CacheResult> for Option<{nt}> {{
    fn from(value: CacheResult) -> Self {{
        match value {{
            CacheResult::{nt}(inner) => inner,
            _ => panic!("cache not matched")
        }}
    }}
}}
'''

visitor_template = '''
fn {nt_lower}(&mut self, {nt_lower}: {nt}) {{
    todo!()
}}
'''

_, types = interface.generate(path, False)

# generate node.rs
fmt_node_list = [node_template.format(nt=each) for each in types]
node_body = '\n'.join(fmt_node_list)
node = header + '''
use crate::cache::CacheResult;
use std::fmt::{{Debug, Formatter}};
''' + node_body


# generate visitor.rs
fmt_visitor_list = [visitor_template.format(nt_lower=each.lower(), nt=each) for each in types]
visitor_body = '\n'.join(fmt_visitor_list)
visitor = header + '''
use crate::node::{{{}}};

pub struct Visitor {{
    pub indent: usize,
    pub output: Vec<String>,
}}

macro_rules! indent {{
    ($self:ident, $inside:block) => {{
        $self.indent += 1;
        $inside
        $self.indent -= 1;
    }};
}}

macro_rules! lp {{
    ($self:ident, $($arg:tt)*) => {{
        $self.output.push(format!("{{}}{{}}", "    ".repeat($self.indent), format!($($arg)*)))
    }};
}}

macro_rules! p {{
    ($self:ident, $($arg:tt)*) => {{
        if let Some(last) = $self.output.last_mut() {{
            last.push_str(format!($($arg)*).as_str())
        }} else {{
            lp!($self, $($arg)*)
        }}
    }};
}}
'''.format(', '.join(types)) + visitor_body

print(node)
print(visitor)
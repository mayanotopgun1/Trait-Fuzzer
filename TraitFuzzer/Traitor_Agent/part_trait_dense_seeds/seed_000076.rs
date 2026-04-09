#![feature(specialization)]

pub trait Backend {
    type DescriptorSetLayout;
}

default impl<B> Backend for B {
    type DescriptorSetLayout = ();
}

impl Backend for Back {
    type DescriptorSetLayout = u32;
}

pub struct HalSetLayouts {
    vertex_layout: <Back as Backend>::DescriptorSetLayout,
}

trait IterExt<DSL> {
    fn iter(self) -> DSL;
}

default impl<I, DSL> IterExt<DSL> for I {
    default fn iter(self) -> DSL {
        unimplemented!()
    }
}

impl IterExt<<Back as Backend>::DescriptorSetLayout> for HalSetLayouts {
    fn iter(self) -> <Back as Backend>::DescriptorSetLayout {
        self.vertex_layout
    }
}

pub struct Back;

fn main() {}
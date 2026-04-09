#![allow(dead_code)]

trait PrinterSupport<'ast> {
    fn ast_map(&self) -> Option<&'ast usize> { None }
}

trait ExtendedPrinterSupport<'ast>: PrinterSupport<'ast> + 'ast {
    fn is_annotation_empty(&self) -> bool {
        self.ast_map().is_none()
    }
}

impl<'ast, T: PrinterSupport<'ast> + 'ast> ExtendedPrinterSupport<'ast> for T {}

struct NoAnn<'ast> {
    f: Option<&'ast usize>
}

impl<'ast> PrinterSupport<'ast> for NoAnn<'ast> {
    fn ast_map(&self) -> Option<&'ast usize> {
        self.f
    }
}

fn foo<'ast, G>(f: Option<&'ast usize>, g: G)
where
    G: FnOnce(&dyn ExtendedPrinterSupport<'ast>),
{
    let annotation = NoAnn { f };
    g(&annotation)
}

fn main() {}
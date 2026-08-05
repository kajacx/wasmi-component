use std::rc::Rc;

use crate::lib_structs::FuncStorage;

#[derive(Debug, Clone)]
pub struct Component {
    pub index: usize, // TODO: readonly
    pub(crate) core_module: wasmi::Module,

    pub(crate) imported_funcs: Rc<FuncStorage>,
    pub(crate) exported_funcs: Rc<FuncStorage>,
}

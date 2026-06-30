pub struct ExampleExports {
    pub funcs_add_s32: ::wasmi_component::TypedFunc<(i32, i32), i32>,
}
pub fn instantiate_example_world(store_ctx: &mut ::wasmi_component::Store, component: &::wasmi_component::Component) -> ExampleExports

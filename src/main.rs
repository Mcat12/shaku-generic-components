use async_trait::async_trait;
use shaku::{module, Component, HasComponent, Interface};

pub trait Executor: Interface + Default {}

#[derive(Default)]
pub struct SqlConnection;

impl Executor for SqlConnection {}

#[derive(Clone, Default)]
pub struct DbPool;

impl Executor for DbPool {}

#[async_trait]
pub trait MyService: Interface {
    async fn foo(&self) -> u16;
}

#[derive(Component)]
#[shaku(interface = MyService)]
pub struct MyServiceImpl<E>
where
    E: Executor,
{
    #[allow(dead_code)]
    executor: E,
}

#[async_trait]
impl<E> MyService for MyServiceImpl<E>
where
    E: Executor,
{
    async fn foo(&self) -> u16 {
        1337
    }
}

module! {
    MyModule<E: Executor> {
        components = [MyServiceImpl<E>],
        providers = []
    }
}

fn build_module<E>(executor: E) -> MyModule<E>
where
    E: Executor,
{
    MyModule::builder()
        .with_component_parameters::<MyServiceImpl<E>>(MyServiceImplParameters { executor })
        .build()
}

#[async_std::main]
async fn main() {
    let pool = DbPool {};

    let module = build_module(pool.clone());
    let my_service: &dyn MyService = module.resolve_ref();
    let result = my_service.foo().await;
    println!("Foo: {}", result);
}

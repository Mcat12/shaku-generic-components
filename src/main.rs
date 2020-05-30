#[macro_use]
extern crate shaku_derive;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate async_std;

use shaku::{module, Component, Interface, HasComponent};

pub trait Executor {}

pub struct SqlConnection;

impl Executor for SqlConnection {}

#[derive(Clone)]
pub struct DbPool;

impl Executor for DbPool {}

#[async_trait]
pub trait MyService: Interface {
    async fn foo(&self) -> u16;
}

#[derive(Component)]
#[shaku(interface = MyService)]
pub struct MyServiceImpl<E>
    where E: Executor,
{
    executor: E,
}

impl<E> MyService for MyServiceImpl<E>
    where E: Executor,
{
    async fn foo(&self) -> u16 {
        1337
    }
}

module! {
    MyModule {
        components = [MyServiceImpl],
        providers = []
    }
}

fn build_module<E>(executor: E)
    where E: Executor,
{
    MyModule::builder()
        .with_component_parameters::<MyServiceImpl<E>>(MyServiceImplParameters {
            executor: pool.clone(),
        })
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

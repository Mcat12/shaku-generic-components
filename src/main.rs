use async_trait::async_trait;
use shaku::{Component, HasComponent, Interface, Module, ModuleBuildContext, ModuleBuilder};
use std::marker::PhantomData;
use std::sync::Arc;

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

// #[derive(Component)]
// #[shaku(interface = MyService)]
pub struct MyServiceImpl<E>
where
    E: Executor,
{
    #[allow(dead_code)]
    executor: E,
}

impl<E: Executor, M: Module> Component<M> for MyServiceImpl<E> {
    type Interface = dyn MyService;
    type Parameters = E;

    fn build(_context: &mut ModuleBuildContext<M>, params: E) -> Box<dyn MyService> {
        Box::new(Self { executor: params })
    }
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

// module! {
//     MyModule {
//         components = [MyServiceImpl],
//         providers = []
//     }
// }

struct MyModule<E: Executor> {
    my_service: Arc<dyn MyService>,
    _phantom: PhantomData<E>,
}

impl<E: Executor + Interface> Module for MyModule<E> {
    type Submodules = ();

    fn build(context: &mut ModuleBuildContext<Self>) -> Self {
        Self {
            my_service: Self::build_component(context),
            _phantom: PhantomData,
        }
    }
}

impl<E: Executor + Interface> HasComponent<dyn MyService> for MyModule<E> {
    fn build_component(context: &mut ModuleBuildContext<Self>) -> Arc<dyn MyService> {
        context.build_component::<MyServiceImpl<E>>()
    }

    fn resolve(&self) -> Arc<dyn MyService> {
        Arc::clone(&self.my_service)
    }

    fn resolve_ref(&self) -> &dyn MyService {
        Arc::as_ref(&self.my_service)
    }

    fn resolve_mut(&mut self) -> Option<&mut dyn MyService> {
        Arc::get_mut(&mut self.my_service)
    }
}

fn build_module<E>(executor: E) -> MyModule<E>
where
    E: Executor,
{
    ModuleBuilder::with_submodules(())
        .with_component_parameters::<MyServiceImpl<E>>(executor)
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

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use crate::application::di::providable_trait::Providable;
use crate::application::di::provider_trait::{Provider, ReferenceProvider};

#[derive(Debug, Default)]
pub struct Container {
    providers: HashMap<TypeId, Arc<dyn Any>>
}

unsafe impl Send for Container { }
unsafe impl Sync for Container { }

impl Container {
    pub fn install_value_provider
    <TypeProvided: Providable, PROVIDER: 'static + Provider<TypeProvided = TypeProvided>>
    (&mut self, provider: PROVIDER) {
        self.providers.insert(provider.id(),
                              Arc::new(Self::box_provider(provider)));
    }

    pub fn install_reference_provider
    <ReferenceType: Providable, PROVIDER: 'static + ReferenceProvider<RefProvided = ReferenceType>>
    (&mut self, provider: PROVIDER) {
        self.providers.insert(provider.id(),
                              Arc::new(Self::box_ref_provider(provider)));
    }

    pub fn get<TypeToGet: Providable>(&self) -> Result<TypeToGet, String> {
        let provider = self.providers
            .get(&Self::get_id::<TypeToGet>())
            .expect("Error getting provider from container.")
            .downcast_ref::<Box<dyn Provider<TypeProvided = TypeToGet>>>()
            .expect("Error casting provider.");
        provider.provide(&self)
    }

    pub fn get_ref<RefToGet: 'static>(&self) -> Result<&RefToGet, String> {
        let provider = self.providers
            .get(&Self::get_id::<&RefToGet>())
            .expect("Error getting provider from container.")
            .downcast_ref::<Box<dyn ReferenceProvider<RefProvided = RefToGet>>>()
            .expect("Error casting provider.");
        provider.provide(&self)
    }

    fn get_id<T: 'static>() -> TypeId {
        TypeId::of::<T>()
    }

    // We want box references to the providers.
    fn box_provider<T: 'static, P: 'static + Provider<TypeProvided = T>>(
        provider: P,
    ) -> Box<dyn Provider<TypeProvided = T>> {
        Box::new(provider)
    }

    // We want box references to the providers.
    fn box_ref_provider<T: 'static, P: 'static + ReferenceProvider<RefProvided = T>>(
        provider: P,
    ) -> Box<dyn ReferenceProvider<RefProvided = T>> {
        Box::new(provider)
    }
}


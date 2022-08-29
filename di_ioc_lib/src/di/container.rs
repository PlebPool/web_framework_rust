use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use crate::di::providable_trait::Providable;
use crate::di::provider_trait::{Provider, ReferenceProvider};

pub enum ProviderError {
    ProviderMissing,
    ProviderCastFailed,
    ProviderFailed
}

#[derive(Debug, Default)]
pub struct Container {
    providers: HashMap<TypeId, Arc<dyn Any>>
}

/// Telling the compiler that the Container is safe to send to other threads.
unsafe impl Send for Container { }
unsafe impl Sync for Container { }

// TODO: Renovate error handling.
impl Container {
    pub fn install_value_provider
    <TypeProvided: Providable, PROVIDER: 'static + Provider<TypeProvided = TypeProvided>>
    (&mut self, provider: PROVIDER) {
        self.providers.insert(provider.id_of_type_provided(),
                              Arc::new(Self::box_provider(provider)));
    }

    pub fn install_reference_provider
    <ReferenceType: Providable, PROVIDER: 'static + ReferenceProvider<RefProvided = ReferenceType>>
    (&mut self, provider: PROVIDER) {
        self.providers.insert(provider.id_of_reference_provided(),
                              Arc::new(Self::box_ref_provider(provider)));
    }

    pub fn get<TypeToGet: Providable>(&self) -> Result<TypeToGet, ProviderError> {
        let provider = self.providers
            .get(&Self::get_id::<TypeToGet>())
            .ok_or_else(|| ProviderError::ProviderMissing)?
            .downcast_ref::<Box<dyn Provider<TypeProvided = TypeToGet>>>()
            .ok_or_else(|| ProviderError::ProviderCastFailed)?;
        let provider_result = provider.provide(&self);
        match provider_result {
            Ok(t) => { Ok(t) },
            Err(e) => { panic!("Provider Failed: {}", e) }
        }
    }

    pub fn get_ref<RefToGet: 'static>(&self) -> Result<&RefToGet, ProviderError> {
        let provider = self.providers
            .get(&Self::get_id::<&RefToGet>())
            .ok_or_else(|| ProviderError::ProviderMissing)?
            .downcast_ref::<Box<dyn ReferenceProvider<RefProvided = RefToGet>>>()
            .ok_or_else(|| ProviderError::ProviderCastFailed)?;
        let provider_result: Result<&RefToGet, String> = provider.provide(&self);
        match provider_result {
            Ok(t) => { Ok(t) },
            Err(e) => { panic!("Provider Failed: {}", e) }
        }
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


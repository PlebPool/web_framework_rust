use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use crate::di::providable_trait::Providable;
use crate::di::provider_trait::{Provider, ReferenceProvider};

#[derive(Debug)]
pub enum ProviderError {
    ProviderMissing,
    ProviderCastFailed
}

#[derive(Debug)]
pub struct IocContainer {
    providers: HashMap<TypeId, Arc<dyn Any>>
}

impl Default for IocContainer {
    fn default() -> Self {
        let _ = env_logger::try_init();
        Self {
            providers: HashMap::default()
        }
    }
}

/// Telling the compiler that the Container is safe to send to other threads.
unsafe impl Send for IocContainer { }
unsafe impl Sync for IocContainer { }

impl IocContainer {
    /// It takes a provider and installs it into the registry
    ///
    /// Arguments:
    ///
    /// * `provider`: The provider to install.
    pub fn install_value_provider
    <TypeProvided: Providable, PROVIDER: 'static + Provider<TypeProvided = TypeProvided>>
    (&mut self, provider: PROVIDER) {
        self.providers.insert(provider.id_of_type_provided(),
                              Arc::new(Self::box_provider(provider)));
    }

    /// `install_reference_provider` installs a reference provider
    ///
    /// Arguments:
    ///
    /// * `provider`: The provider to install.
    pub fn install_reference_provider
    <ReferenceType: Providable, PROVIDER: 'static + ReferenceProvider<RefProvided = ReferenceType>>
    (&mut self, provider: PROVIDER) {
        self.providers.insert(provider.id_of_reference_provided(),
                              Arc::new(Self::box_ref_provider(provider)));
    }

    /// "Get the provider for the type we want, then call the provider's provide function, and return
    /// the result."
    ///
    /// The first thing we do is get the provider for the type we want. We do this by getting the
    /// provider id for the type we want, then getting the provider from the providers map. If the
    /// provider is missing, we return a ProviderError::ProviderMissing error
    ///
    /// Returns:
    ///
    /// A Result<TypeToGet, ProviderError>
    pub fn get<TypeToGet: Providable>(&self) -> Result<TypeToGet, ProviderError> {
        let provider = self.providers
            .get(&Self::get_id::<TypeToGet>())
            .ok_or_else(|| ProviderError::ProviderMissing)?
            .downcast_ref::<Box<dyn Provider<TypeProvided = TypeToGet>>>()
            .ok_or_else(|| ProviderError::ProviderCastFailed)?;
        let provider_result: Result<TypeToGet, String> = provider.provide(&self);
        match provider_result {
            Ok(t) => { Ok(t) },
            Err(e) => { panic!("Provider Failed: {}", e) }
        }
    }

    /// > This function takes a reference to a `ReferenceProvider` and returns a reference to the type
    /// of `RefProvided` that the `ReferenceProvider` provides
    ///
    /// Returns:
    ///
    /// A reference to the type that was requested.
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

    /// It returns a unique identifier for a given type
    ///
    /// Returns:
    ///
    /// A TypeId
    fn get_id<T: 'static>() -> TypeId {
        TypeId::of::<T>()
    }

    /// Returns a box reference referencing a value provider.
    fn box_provider<T: 'static, P: 'static + Provider<TypeProvided = T>>
    (provider: P, ) -> Box<dyn Provider<TypeProvided = T>> {
        Box::new(provider)
    }

    /// Returns a box reference referencing a reference provider.
    fn box_ref_provider<T: 'static, P: 'static + ReferenceProvider<RefProvided = T>>
    (provider: P, ) -> Box<dyn ReferenceProvider<RefProvided = T>> {
        Box::new(provider)
    }
}


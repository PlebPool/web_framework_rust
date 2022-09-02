use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::di::ioc_container::IocContainer;
use crate::di::providable_trait::Providable;

/// Trait defining a value provider. To be used by container.
pub trait Provider {
    type TypeProvided: 'static;

    /// A function that takes a container and returns a result of type `Self::TypeProvided` or a string.
    fn provide(&self, container: &IocContainer) -> Result<Self::TypeProvided, String>;

    /// `TypeId::of::<Self::TypeProvided>()`
    ///
    /// Returns:
    ///
    /// The TypeId of the type that the trait is being implemented for.
    fn id_of_type_provided(&self) -> TypeId { // Default impl.
        TypeId::of::<Self::TypeProvided>()
    }
}

/// Trait defining a reference provider. To be used by container.
pub trait ReferenceProvider {
    type RefProvided: 'static;

    /// A function that takes a container and returns a result of type `Self::TypeProvided` or a string.
    fn provide(&self, container: &IocContainer) -> Result<&Self::RefProvided, String>;

    /// `TypeId::of::<&Self::RefProvided>()`
    ///
    /// The `TypeId` struct is a type that is used to identify a type
    ///
    /// Returns:
    ///
    /// The type id of the type of the reference that the trait object will provide.
    fn id_of_reference_provided(&self) -> TypeId { // Default impl.
        TypeId::of::<&Self::RefProvided>()
    }
}

/// Implementing the `ReferenceProvider` trait for the `Arc<T>` type.
impl <T: Providable> ReferenceProvider for Arc<T> {
    type RefProvided = T;

    fn provide(&self, _: &IocContainer) -> Result<&Self::RefProvided, String> {
        Ok(&self)
    }
}

/// Implementing the `Providable` trait for the `HashMap<K, V>` type.
impl <K: 'static, V: 'static> Providable for HashMap<K, V> { }

/// Implementing the `Providable` trait for the `Vec<T>` type.
impl <T: 'static> Providable for Vec<T> { }

/// Implementing the `Provider` trait for the `Arc<T>` type.
impl <T: Providable> Provider for Arc<T> {
    type TypeProvided = Self;

    fn provide(&self, _: &IocContainer) -> Result<Self::TypeProvided, String> {
        Ok(Arc::clone(&self))
    }
}

/// Implementing the `ReferenceProvider` trait for the `Box<T>` type.
impl <T: Providable> ReferenceProvider for Box<T> {
    type RefProvided = T;

    fn provide(&self, _: &IocContainer) -> Result<&Self::RefProvided, String> {
        Ok(&self)
    }
}

/// Implementing the `ReferenceProvider` trait for the `Rc<T>` type.
impl <T: Providable> ReferenceProvider for Rc<T> {
    type RefProvided = T;

    fn provide(&self, _: &IocContainer) -> Result<&Self::RefProvided, String> {
        Ok(&self)
    }
}
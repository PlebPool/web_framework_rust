use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::application::di::container::Container;
use crate::application::di::providable_trait::Providable;

/// Trait defining a value provider. To be used by container.
pub trait Provider {
    type TypeProvided: 'static;

    /// A function that takes a container and returns a result of type `Self::TypeProvided` or a string.
    fn provide(&self, container: &Container) -> Result<Self::TypeProvided, String>;

    /// `TypeId::of::<Self::TypeProvided>()`
    ///
    /// Returns:
    ///
    /// The TypeId of the type that the trait is being implemented for.
    fn id(&self) -> TypeId { // Default impl.
        TypeId::of::<Self::TypeProvided>()
    }
}

/// Trait defining a reference provider. To be used by container.
pub trait ReferenceProvider {
    type RefProvided: 'static;

    /// A function that takes a container and returns a result of type `Self::TypeProvided` or a string.
    fn provide(&self, container: &Container) -> Result<&Self::RefProvided, String>;

    /// `TypeId::of::<&Self::RefProvided>()`
    ///
    /// The `TypeId` struct is a type that is used to identify a type
    ///
    /// Returns:
    ///
    /// The type id of the type of the reference that the trait object will provide.
    fn id(&self) -> TypeId { // Default impl.
        TypeId::of::<&Self::RefProvided>()
    }
}

impl <T: Providable> ReferenceProvider for Arc<T> {
    type RefProvided = T;

    fn provide(&self, _: &Container) -> Result<&Self::RefProvided, String> {
        Ok(&self)
    }
}

impl <K: 'static, V: 'static> Providable for HashMap<K, V> { }

impl <T: Providable> Provider for Arc<T> {
    type TypeProvided = Self;

    fn provide(&self, _: &Container) -> Result<Self::TypeProvided, String> {
        Ok(Arc::clone(&self))
    }
}

impl <T: Providable> ReferenceProvider for Box<T> {
    type RefProvided = T;

    fn provide(&self, _: &Container) -> Result<&Self::RefProvided, String> {
        Ok(&self)
    }
}

impl <T: Providable> ReferenceProvider for Rc<T> {
    type RefProvided = T;

    fn provide(&self, _: &Container) -> Result<&Self::RefProvided, String> {
        Ok(&self)
    }
}
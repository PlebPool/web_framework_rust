use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::application::di::container::Container;
use crate::application::di::providable_trait::Providable;

pub trait Provider {
    type TypeProvided: 'static;

    fn provide(&self, container: &Container) -> Result<Self::TypeProvided, String>;

    fn id(&self) -> TypeId { // Default impl.
        TypeId::of::<Self::TypeProvided>()
    }
}

pub trait ReferenceProvider {
    type RefProvided: 'static;

    fn provide(&self, container: &Container) -> Result<&Self::RefProvided, String>;

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
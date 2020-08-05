use crate::observer::*;
use crate::traits::*;

pub struct ConcreteSubject<'a, T: Observer> {
    observers: Vec<&'a mut T>,
}

impl<'a, T: Observer + PartialEq> ConcreteSubject<'a, T> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: Observer + PartialEq> Subject<'a, T> for ConcreteSubject<'a, T> {
    fn add_listener(&mut self, observer: &'a mut T) {
        self.observers.push(observer);
    }
    fn remove_listener(&mut self, observer: &'a mut T) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
        }
    }
    fn notify_observers(&mut self) {
        self.observers.iter_mut()
            .for_each(|x| x.on_notify());
    }
}

pub struct ConcreteRadiusSubject<'a, T: RadiusObserver> {
    radius: i32,
    observers: Vec<&'a mut T>,
}

impl<'a, T: RadiusObserver + PartialEq> ConcreteRadiusSubject<'a, T> {
    pub fn new(radius: i32) -> Self {
        Self {
            radius,
            observers: Vec::new(),
        }
    }
}

impl<'a, T: RadiusObserver + PartialEq> RadiusSubject<'a, T> for ConcreteRadiusSubject<'a, T> {
    fn add_listener(&mut self, observer: &'a mut T) {
        self.observers.push(observer);
    }
    fn remove_listener(&mut self, observer: &'a mut T) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
        }
    }
    fn notify_observers(&mut self) {
        let radius = self.radius;
        self.observers.iter_mut()
            .for_each(|x| x.on_notify(radius));
    }
}

pub struct ConcreteDynamicSubject<'a> {
    observers:Vec<Box<&'a mut dyn Observer>>,
}

impl<'a> ConcreteDynamicSubject<'a> {
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }
}

impl<'a> DynamicSubject<'a> for ConcreteDynamicSubject<'a> {
    fn add_listener(&mut self, observer: &'a mut dyn Observer) {
        self.observers.push(Box::new(observer));
    }
    fn notify_observers(&mut self) {
        self.observers.iter_mut()
            .for_each(|x| x.on_notify());
    }

}

pub struct NewConcreteSubject<T: Observer> {
    observers: Vec<T>,
}

impl<T: Observer + PartialEq> NewConcreteSubject<T> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl<T: Observer + PartialEq> NewSubject<T> for NewConcreteSubject<T> {
    fn add_listener(&mut self, observer: T) {
        self.observers.push(observer);
    }
    fn remove_listener(&mut self, observer: T) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
        }
    }
    fn notify_observers(&mut self) {
        self.observers.iter_mut()
            .for_each(|x| x.on_notify());
    }
}

pub struct ConcreteEntitySubject<T: EntityObserver> {
    observers: Vec<T>,
}

impl<T: EntityObserver + PartialEq> ConcreteEntitySubject<T> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl<T: EntityObserver + PartialEq> EntitySubject<T> for ConcreteEntitySubject<T> {
    fn add_listener(&mut self, observer: T) {
        self.observers.push(observer);
    }
    fn remove_listener(&mut self, observer: T) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
        }
    }
    fn notify_observers(&self, entity: &mut Entity, radius: i32) {
        self.observers.iter()
            .for_each(|x| x.on_notify(entity, radius));
    }

}

pub struct ConcreteAssociatedTypeSubject<T: AssociatedTypeObserver> {
    observers: Vec<T>,
}

impl<T: AssociatedTypeObserver + PartialEq> ConcreteAssociatedTypeSubject<T> {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl<T: AssociatedTypeObserver + PartialEq> AssociatedTypeSubject for ConcreteAssociatedTypeSubject<T> {
    type Observer = T;
    type Notification = T::Notification;

    fn add_listener(&mut self, observer: Self::Observer) {
        self.observers.push(observer);
    }

    fn remove_listener(&mut self, observer: Self::Observer) {
        if let Some(index) = self.observers.iter().position(|x| *x == observer) {
            self.observers.remove(index);
        }
    }

    fn notify_observers_borrow(&self, notification: Option<&Self::Notification>) {
        self.observers.iter()
            .for_each(|x| {
                if let Some(notification) = notification {
                    x.on_notify_borrow(notification);
                }
            });
    }
}
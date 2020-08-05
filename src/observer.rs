use std::marker::PhantomData;

use crate::traits::*;

#[derive(PartialEq, Clone, Copy)]
pub struct ConcreteObserver {
    pub id: i32,
}

impl Observer for ConcreteObserver {
    fn on_notify(&mut self) {
        self.id += 1;
        println!("id: {:?} got event", self.id );
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct AnotherConcreteObserver {
    pub id: i32,
}

impl Observer for AnotherConcreteObserver {
    fn on_notify(&mut self) {
        self.id += 1;
        println!("id: {:?} got event", self.id );
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ConcreteObserverEnum {
    ConcreteObserver(ConcreteObserver),
    AnotherConcreteObserver(AnotherConcreteObserver),
}

impl Observer for ConcreteObserverEnum {
    fn on_notify(&mut self) {
        match self {
            ConcreteObserverEnum::ConcreteObserver(observer) => observer.on_notify(),
            ConcreteObserverEnum::AnotherConcreteObserver(observer) => observer.on_notify(),

        }
    }
}

#[derive(PartialEq)]
pub struct ConcreteRadiusObserver {
    pub radius: i32,
}

impl RadiusObserver for ConcreteRadiusObserver {
    fn on_notify(&mut self, radius: i32) {
        self.radius = radius ;
        println!("radius: {:?} new radius", self.radius );
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct ConcreteEntityObserver {}

impl EntityObserver for ConcreteEntityObserver {
    fn on_notify(&self, entity: &mut Entity, radius: i32) {
        entity.radius = radius;
    }

}

#[derive(PartialEq, Clone, Copy)]
pub struct AnotherConcreteEntityObserver {}

impl EntityObserver for AnotherConcreteEntityObserver {
    fn on_notify(&self, entity: &mut Entity, radius: i32) {
        println!("{:?} has radius {:?}", entity, radius);
    }

}

#[derive(Debug)]
pub struct Entity{ 
    pub radius: i32
}

#[derive(PartialEq, Clone, Copy)]
pub enum ConcreteEntityObserverEnum {
    ConcreteEntityObserver(ConcreteEntityObserver),
    AnotherConcreteEntityObserver(AnotherConcreteEntityObserver),
}

impl EntityObserver for ConcreteEntityObserverEnum {
    fn on_notify(&self, entity: &mut Entity, radius: i32) {
        match self {
            ConcreteEntityObserverEnum::ConcreteEntityObserver(observer) => {
                observer.on_notify(entity, radius);
            }
            ConcreteEntityObserverEnum::AnotherConcreteEntityObserver(observer) => {
                observer.on_notify(entity, radius);
            }

        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct ConcreteAssociatedTypeObserver<T: std::fmt::Display>{
    _fantum: PhantomData<T>,
}

impl<T: std::fmt::Display> ConcreteAssociatedTypeObserver<T> {
    pub fn new() -> Self {
        Self {
            _fantum: PhantomData{}
        }
    }
}

impl<T: std::fmt::Display> AssociatedTypeObserver for ConcreteAssociatedTypeObserver<T> {
    type Notification = T;
    fn on_notify(&mut self, notification: &mut Self::Notification) {
        println!("CATO {}", notification);
    }
    fn on_notify_borrow(&self, notification: &Self::Notification) {
        println!("CATO {}", notification);
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct AnotherConcreteAssociatedTypeObserver<T: std::fmt::Display>{
    _fantum: PhantomData<T>,
}

impl<T: std::fmt::Display> AnotherConcreteAssociatedTypeObserver<T> {
    pub fn new() -> Self {
        Self {
            _fantum: PhantomData{}
        }
    }
}

impl<T: std::fmt::Display> AssociatedTypeObserver for AnotherConcreteAssociatedTypeObserver<T> {
    type Notification = T;
    fn on_notify(&mut self, notification: &mut Self::Notification) {
        println!("ACATO {}", notification);
    }
    fn on_notify_borrow(&self, notification: &Self::Notification) {
        println!("ACATO {}", notification);
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ConcreteAssociatedTypeObserverEnum<T: std::fmt::Display>{
    CATO(ConcreteAssociatedTypeObserver<T>),
    ACATO(AnotherConcreteAssociatedTypeObserver<T>),
}

impl<T: std::fmt::Display> AssociatedTypeObserver for ConcreteAssociatedTypeObserverEnum<T> {
    type Notification = T;
    fn on_notify(&mut self, notification: &mut Self::Notification) {
        match self {
            ConcreteAssociatedTypeObserverEnum::CATO(observer) => {
                observer.on_notify(notification)
            },
            ConcreteAssociatedTypeObserverEnum::ACATO(observer) => {
                observer.on_notify(notification)
            },
        }
    }

    fn on_notify_borrow(&self, notification: &Self::Notification) {
        match self {
            ConcreteAssociatedTypeObserverEnum::CATO(observer) => {
                observer.on_notify_borrow(notification)
            },
            ConcreteAssociatedTypeObserverEnum::ACATO(observer) => {
                observer.on_notify_borrow(notification)
            },
        }
    }

}
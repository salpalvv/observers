use crate::observer::*;

pub trait Observer {
    fn on_notify(&mut self);
}

pub trait Subject<'a, T: Observer> {
    fn add_listener(&mut self, observer: &'a mut T);
    fn remove_listener(&mut self, observer: &'a mut T);
    fn notify_observers(&mut self);
}

pub trait DynamicSubject<'a> {
    fn add_listener(&mut self, observer: &'a mut dyn Observer);
    fn notify_observers(&mut self);
}

pub trait NewSubject<T: Observer> {
    fn add_listener(&mut self, observer: T);
    fn remove_listener(&mut self, observer: T);
    fn notify_observers(&mut self);
}

pub trait RadiusObserver {
    fn on_notify(&mut self, radius: i32);
}

pub trait RadiusSubject<'a, T: RadiusObserver> {
    fn add_listener(&mut self, observer: &'a mut T);
    fn remove_listener(&mut self, observer: &'a mut T);
    fn notify_observers(&mut self);
}

pub trait EntityObserver {
    fn on_notify(&self, entity: &mut Entity, radius: i32);
}

pub trait EntitySubject<T: EntityObserver> {
    fn add_listener(&mut self, observer: T);
    fn remove_listener(&mut self, observer: T);
    fn notify_observers(&self, entity: &mut Entity, radius: i32);
}

pub trait AssociatedTypeObserver {
    type Notification;
    fn on_notify(&mut self, _: &mut Self::Notification){}
    fn on_notify_borrow(&self, _: &Self::Notification){}
}

pub trait AssociatedTypeSubject {
    type Observer;
    type Notification;

    fn add_listener(&mut self, observer: Self::Observer);
    fn remove_listener(&mut self, observer: Self::Observer);
    fn notify_observers(&self, _: Option<&mut Self::Notification>){}
    fn notify_observers_mut(&mut self, _: Option<&mut Self::Notification>) {}
    fn notify_observers_borrow(&self, _: Option<&Self::Notification>){}
    
}

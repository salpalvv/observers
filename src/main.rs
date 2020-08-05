use std::marker::PhantomData;

mod observer;
mod subject;
mod traits;

use observer::*;
use subject::*;
use traits::*;

fn main() {
    let mut subject = ConcreteSubject::new();
    let mut observer_a = ConcreteObserver{ id: 1 };
    let mut observer_b = ConcreteObserver{ id: 2 };

    subject.add_listener(&mut observer_a);
    subject.add_listener(&mut observer_b);

    subject.notify_observers();
    subject.notify_observers();
    subject.notify_observers();

    let mut r_subject = ConcreteRadiusSubject::new(10);
    let mut r_observer_a = ConcreteRadiusObserver{ radius: 0 };
    let mut r_observer_b = ConcreteRadiusObserver{ radius: 0 };

    r_subject.add_listener(&mut r_observer_a);
    r_subject.add_listener(&mut r_observer_b);

    r_subject.notify_observers();

    let mut d_subject = ConcreteDynamicSubject::new();
    let mut d_observer_a = ConcreteObserver{ id: 1 };
    let mut d_observer_b = AnotherConcreteObserver{ id: 2 };

    d_subject.add_listener(&mut d_observer_a);
    d_subject.add_listener(&mut d_observer_b);

    d_subject.notify_observers();
    d_subject.notify_observers();
    d_subject.notify_observers();

    let mut enum_subject = ConcreteSubject::new();
    let mut o_a = ConcreteObserverEnum::ConcreteObserver(ConcreteObserver{ id: 10});
    let mut o_b = ConcreteObserverEnum::AnotherConcreteObserver(AnotherConcreteObserver{ id: 20});

    enum_subject.add_listener(&mut o_a);
    enum_subject.add_listener(&mut o_b);
    // can't do this because you can't mutably borrow more than once.
    //enum_subject.remove_listener(&mut o_b);

    enum_subject.notify_observers();

    let mut new_subject = NewConcreteSubject::new();
    let new_o_a = ConcreteObserverEnum::ConcreteObserver(ConcreteObserver{ id: 100});
    let new_o_b = ConcreteObserverEnum::AnotherConcreteObserver(AnotherConcreteObserver{ id: 200});

    new_subject.add_listener(new_o_a);
    new_subject.add_listener(new_o_b);
    new_subject.remove_listener(new_o_b);
    new_subject.notify_observers();

    let mut entity = Entity{ radius: 10 };
    let mut entity_subject = ConcreteEntitySubject::new();
    let entity_o_a = ConcreteEntityObserverEnum::ConcreteEntityObserver(ConcreteEntityObserver{});
    let entity_o_b = ConcreteEntityObserverEnum::AnotherConcreteEntityObserver(AnotherConcreteEntityObserver{});

    entity_subject.add_listener(entity_o_a);
    entity_subject.add_listener(entity_o_b);
    entity_subject.remove_listener(entity_o_b);
    entity_subject.add_listener(entity_o_b);
    entity_subject.notify_observers(&mut entity, 20);

    let notification = Notification{ radius: String::from("DO IT!") };
    let mut associated_type_subject = ConcreteAssociatedTypeSubject::new();
    let c_a_t_o: ConcreteAssociatedTypeObserver<Notification> = ConcreteAssociatedTypeObserver::new();
    let a_c_a_t_o: AnotherConcreteAssociatedTypeObserver<Notification> = AnotherConcreteAssociatedTypeObserver::new();
    let associated_type_observer_a = ConcreteAssociatedTypeObserverEnum::CATO(c_a_t_o);
    let associated_type_observer_b = ConcreteAssociatedTypeObserverEnum::ACATO(a_c_a_t_o);

    associated_type_subject.add_listener(associated_type_observer_a);
    associated_type_subject.add_listener(associated_type_observer_b);
    // you aren't able to remove because the observers Vec moved the observers_b already so you can't use it anymore.
    //associated_type_subject.remove_listener(associated_type_observer_b);
    //associated_type_subject.add_listener(associated_type_observer_b);
    associated_type_subject.notify_observers_borrow(Some(&notification));

    // just shadowing the previous values to make copy paste faster
    let mut associated_type_reference_subject = CATReferenceSubject::new();
    let c_a_t_o: ConcreteAssociatedTypeObserver<Notification> = ConcreteAssociatedTypeObserver::new();
    let a_c_a_t_o: AnotherConcreteAssociatedTypeObserver<Notification> = AnotherConcreteAssociatedTypeObserver::new();
    let associated_type_observer_a = ConcreteAssociatedTypeObserverEnum::CATO(c_a_t_o);
    let associated_type_observer_b = ConcreteAssociatedTypeObserverEnum::ACATO(a_c_a_t_o);
    // taking references to the observer so i can add and remove 
    associated_type_reference_subject.add_listener(&associated_type_observer_a);
    associated_type_reference_subject.add_listener(&associated_type_observer_b);
    associated_type_reference_subject.remove_listener(&associated_type_observer_b);
    associated_type_reference_subject.add_listener(&associated_type_observer_b);
    associated_type_reference_subject.notify_observers_borrow(Some(&notification));
}

#[derive(Debug)]
pub struct Entity{ 
    pub radius: i32
}

#[derive(Debug, PartialEq)]
pub struct Notification{ 
    pub radius: String,
}

impl std::fmt::Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Notification {{ radius: {} }}", self.radius)
    }

}
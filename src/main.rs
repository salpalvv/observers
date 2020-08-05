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
}
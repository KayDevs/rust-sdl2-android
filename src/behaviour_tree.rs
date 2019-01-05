enum State {
    Running,
    Success,
    Failure,
}

struct BehaviourNode {
    status: State,
    behaviour: Behaviour,
}
trait Behaviour {
    fn tick(&self) -> State;
    fn get_children(&self) -> Vec<BehaviourNode>;
}

pub struct Routine {}

pub struct Schedule {}


enum State {
    COROUTINE_DEAD,
    COROUTINE_READY,
    COROUTINE_RUNNING,
    COROUTINE_SUSPEND,
}

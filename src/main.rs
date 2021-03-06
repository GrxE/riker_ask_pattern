use futures::future::RemoteHandle;
use futures::executor::block_on;
use riker::actors::*;
use riker_patterns::ask::ask;

#[derive(Default)]
struct MyActor;

impl Actor for MyActor {
    // The receiving Msg Type is u32
    type Msg = u32;

    fn recv(&mut self,
            ctx: &Context<Self::Msg>,
            msg: Self::Msg,
            sender: Sender) {

        // sender is the Ask, waiting for a message to be sent back to it
        sender.as_ref()
            .unwrap()
            // The receiving Msg Type is u32, the return Msg Type is String, using format! to make a String
            .try_tell(format!("{}", msg * 2), Some(ctx.myself().into()));
    }
}

fn main() {
    let sys = ActorSystem::new().unwrap();

    let props = Props::new::<MyActor>();
    let my_actor = sys.actor_of_props("my-actor", props).unwrap();

    // ask returns a future that automatically is driven
    // to completion by the system.
    // The return Msg Type is String
    let res: RemoteHandle<String> = ask(&sys, &my_actor, 100_u32);  // the receiving Msg Type is u32

    // the result future can be passed to a library or fuction that
    // expects a future, or it can be extracted locally using `block_on`.
    let res = block_on(res);

    println!("The result value is: {}", res);
}
